//!
//! Satex 服务发现和负载均衡的库
//!
//! 参考实现: [`pingora-load-balancing`](https://github.com/cloudflare/pingora/tree/main/pingora-load-balancing)
//!
mod background;
pub mod discovery;
pub mod health_check;
pub mod registry;
pub mod selector;

use crate::discovery::Discovery;
use crate::health_check::health::Health;
use crate::health_check::HealthCheck;
use crate::selector::{ArcSelector, BackendIter, Selector};
use arc_swap::ArcSwap;
use derivative::Derivative;
use http::Extensions;
use satex_core::Error;
use std::collections::{BTreeSet, HashMap};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::net::{AddrParseError, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::spawn;
use tracing::{info, warn};

#[derive(Derivative)]
#[derivative(Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Backend {
    ///
    /// 后端服务地址
    ///
    pub addr: SocketAddr,

    ///
    /// 后端服务权重,负载均衡算法会使用到该权重值.
    ///
    pub weight: usize,

    ///
    /// 拓展信息
    ///
    #[derivative(PartialEq = "ignore")]
    #[derivative(PartialOrd = "ignore")]
    #[derivative(Hash = "ignore")]
    #[derivative(Ord = "ignore")]
    pub extension: Extensions,
}

impl Backend {
    pub fn new(addr: impl Into<SocketAddr>) -> Self {
        Self::new_with_weight(addr.into(), 1)
    }

    pub fn new_with_weight(addr: impl Into<SocketAddr>, weight: usize) -> Self {
        Self {
            addr: addr.into(),
            weight,
            extension: Extensions::new(),
        }
    }

    pub(crate) fn key(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
impl FromStr for Backend {
    type Err = AddrParseError;

    fn from_str(addr: &str) -> Result<Self, Self::Err> {
        SocketAddr::from_str(addr).map(Backend::new)
    }
}

impl<'a> TryFrom<(&'a str, usize)> for Backend {
    type Error = AddrParseError;

    fn try_from((addr, weight): (&'a str, usize)) -> Result<Self, Self::Error> {
        let addr = SocketAddr::from_str(addr)?;
        Ok(Backend::new_with_weight(addr, weight))
    }
}

pub struct Backends {
    discovery: Box<dyn Discovery + Send + Sync>,
    health_check: Option<Arc<dyn HealthCheck + Send + Sync>>,
    backends: ArcSwap<BTreeSet<Backend>>,
    health: ArcSwap<HashMap<u64, Health>>,
}

impl Backends {
    pub fn new(discovery: impl Discovery + Send + Sync + 'static) -> Self {
        Self {
            discovery: Box::new(discovery),
            health_check: None,
            backends: Default::default(),
            health: Default::default(),
        }
    }

    pub(crate) fn with_health_check(
        self,
        health_check: impl HealthCheck + Send + Sync + 'static,
    ) -> Self {
        Self {
            health_check: Some(Arc::new(health_check)),
            ..self
        }
    }

    /// Updates backends when the new is different from the current set,
    /// the callback will be invoked when the new set of backend is different
    /// from the current one so that the caller can update the selector accordingly.
    fn do_update<F>(
        &self,
        new_backends: BTreeSet<Backend>,
        enablement: HashMap<u64, bool>,
        callback: F,
    ) where
        F: Fn(Arc<BTreeSet<Backend>>),
    {
        if (**self.backends.load()) != new_backends {
            let old_health = self.health.load();
            let mut new_health = HashMap::with_capacity(new_backends.len());
            for backend in new_backends.iter() {
                let key = backend.key();
                // use the default health if the backend is new
                let health = old_health.get(&key).cloned().unwrap_or_default();

                // override enablement
                if let Some(enabled) = enablement.get(&key) {
                    health.enable(*enabled);
                }
                new_health.insert(key, health);
            }

            // TODO: put this all under 1 ArcSwap so the update is atomic
            // It's important the `callback()` executes first since computing selector backends might
            // be expensive. For example, if a caller checks `backends` to see if any are available
            // they may encounter false positives if the selector isn't ready yet.
            let new_backends = Arc::new(new_backends);
            callback(new_backends.clone());
            self.backends.store(new_backends);
            self.health.store(Arc::new(new_health));
        } else {
            // no backend change, just check enablement
            for (key, backend_enabled) in enablement.iter() {
                // override enablement if set
                // this get should always be Some(_) because we already populate `health`` for all known backends
                if let Some(health) = self.health.load().get(key) {
                    health.enable(*backend_enabled);
                }
            }
        }
    }

    /// Whether a certain [Backend] is ready to serve traffic.
    ///
    /// This function returns true when the backend is both healthy and enabled.
    /// This function returns true when the health check is unset but the backend is enabled.
    /// When the health check is set, this function will return false for the `backend` it
    /// doesn't know.
    pub fn ready(&self, backend: &Backend) -> bool {
        self.health
            .load()
            .get(&backend.key())
            // Racing: return `None` when this function is called between the
            // backend store and the health store
            .map_or(self.health_check.is_none(), |h| h.ready())
    }

    /// Manually set if a [Backend] is ready to serve traffic.
    ///
    /// This method does not override the health of the backend. It is meant to be used
    /// to stop a backend from accepting traffic when it is still healthy.
    ///
    /// This method is noop when the given backend doesn't exist in the service discovery.
    pub fn set_enable(&self, backend: &Backend, enabled: bool) {
        // this should always be Some(_) because health is always populated during update
        if let Some(health) = self.health.load().get(&backend.key()) {
            health.enable(enabled)
        };
    }

    /// Return the collection of the backends.
    pub fn items(&self) -> Arc<BTreeSet<Backend>> {
        self.backends.load_full()
    }

    /// Call the service discovery method to update the collection of backends.
    ///
    /// The callback will be invoked when the new set of backend is different
    /// from the current one so that the caller can update the selector accordingly.
    pub async fn update<F>(&self, callback: F) -> Result<(), Error>
    where
        F: Fn(Arc<BTreeSet<Backend>>),
    {
        let (new_backends, enablement) = self.discovery.discover().await?;
        self.do_update(new_backends, enablement, callback);
        Ok(())
    }

    /// Run health check on all backends if it is set.
    ///
    /// When `parallel: true`, all backends are checked in parallel instead of sequentially
    pub async fn run_health_check(&self, parallel: bool) {
        use crate::health_check::HealthCheck;

        async fn check_and_report(
            backend: &Backend,
            check: &Arc<dyn HealthCheck + Send + Sync>,
            health_table: &HashMap<u64, Health>,
        ) {
            let errored = check.check(backend).await.err();
            if let Some(health) = health_table.get(&backend.key()) {
                let flipped =
                    health.observe_health(errored.is_none(), check.threshold(errored.is_none()));
                if flipped {
                    check.status_change(backend, errored.is_none()).await;
                    if let Some(e) = errored {
                        warn!("{backend:?} becomes unhealthy, {e}");
                    } else {
                        info!("{backend:?} becomes healthy");
                    }
                }
            }
        }

        let Some(health_check) = self.health_check.as_ref() else {
            return;
        };

        let backends = self.backends.load();
        if parallel {
            let health_table = self.health.load_full();
            let jobs = backends.iter().map(|backend| {
                let backend = backend.clone();
                let check = health_check.clone();
                let ht = health_table.clone();
                spawn(async move {
                    check_and_report(&backend, &check, &ht).await;
                })
            });

            futures::future::join_all(jobs).await;
        } else {
            for backend in backends.iter() {
                check_and_report(backend, health_check, &self.health.load()).await;
            }
        }
    }
}

pub struct LoadBalancer {
    backends: Backends,
    selector: ArcSelector,
    update_frequency: Option<Duration>,
    health_check_frequency: Option<Duration>,
    health_check_parallel: bool,
}

impl LoadBalancer {
    pub fn new<S>(backends: Backends, selector: S) -> Self
    where
        S: Selector + Send + Sync + 'static,
        S::Iter: Send + Sync + 'static,
    {
        let selector = ArcSelector::new(selector);
        Self {
            backends,
            selector,
            update_frequency: None,
            health_check_frequency: None,
            health_check_parallel: false,
        }
    }

    pub fn with_health_check(
        mut self,
        health_check: impl HealthCheck + Send + Sync + 'static,
    ) -> Self {
        self.backends = self.backends.with_health_check(health_check);
        self
    }

    pub fn with_update_frequency(mut self, update_frequency: Duration) -> Self {
        self.update_frequency = Some(update_frequency);
        self
    }

    pub fn with_health_check_frequency(mut self, health_check_frequency: Duration) -> Self {
        self.health_check_frequency = Some(health_check_frequency);
        self
    }

    pub fn with_health_check_parallel(mut self, health_check_parallel: bool) -> Self {
        self.health_check_parallel = health_check_parallel;
        self
    }

    /// Run the service discovery and update the selection algorithm.
    ///
    /// This function will be called every `update_frequency` if this [LoadBalancer] instance
    /// is running as a background service.
    pub async fn update(&self) -> Result<(), Error> {
        self.backends
            .update(|backends| self.selector.update(&backends))
            .await
    }

    /// Return the first healthy [Backend] according to the selection algorithm and the
    /// health check results.
    ///
    /// The `key` is used for hash based selection and is ignored if the selection is random or
    /// round robin.
    ///
    /// the `max_iterations` is there to bound the search time for the next Backend. In certain
    /// algorithm like Ketama hashing, the search for the next backend is linear and could take
    /// a lot steps.
    // TODO: consider remove `max_iterations` as users have no idea how to set it.
    pub fn select(&self, key: &[u8]) -> Option<Backend> {
        self.select_with(key, |_, health| health)
    }

    /// Similar to [Self::select], return the first healthy [Backend] according to the selection algorithm
    /// and the user defined `accept` function.
    ///
    /// The `accept` function takes two inputs, the backend being selected and the internal health of that
    /// backend. The function can do things like ignoring the internal health checks or skipping this backend
    /// because it failed before. The `accept` function is called multiple times iterating over backends
    /// until it returns `true`.
    pub fn select_with<F>(&self, key: &[u8], accept: F) -> Option<Backend>
    where
        F: Fn(&Backend, bool) -> bool,
    {
        let mut iter = self.selector.iter(key);
        while let Some(b) = iter.next() {
            if accept(b, self.backends.ready(b)) {
                return Some(b.clone());
            }
        }
        None
    }
}
