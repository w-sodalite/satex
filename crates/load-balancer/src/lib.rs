//!
//! Satex 服务发现和负载均衡的库
//!
//! 参考实现: [`pingora-load-balancing`](https://github.com/cloudflare/pingora/tree/main/pingora-load-balancing)
//!
mod background;
mod load_balancer;

pub mod discovery;
pub mod health_check;
pub mod resolver;
pub mod selector;

pub use load_balancer::LoadBalancer;

use crate::discovery::Discovery;
use crate::health_check::HealthCheck;
use crate::health_check::health::Health;
use arc_swap::ArcSwap;
use derivative::Derivative;
use http::Extensions;
use satex_core::Error;
use std::collections::{BTreeSet, HashMap};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::net::{AddrParseError, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;
use tokio::spawn;
use tracing::{info, warn};

/// 后端服务
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
    /// 创建后端服务实例
    pub fn new(addr: impl Into<SocketAddr>) -> Self {
        Self::new_with_weight(addr.into(), 1)
    }

    /// 创建后端服务实例
    pub fn new_with_weight(addr: impl Into<SocketAddr>, weight: usize) -> Self {
        Self {
            addr: addr.into(),
            weight,
            extension: Extensions::new(),
        }
    }

    /// 计算后端服务的哈希值
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

    /// 当新的后端集合与当前集合不同时更新后端，
    /// 当新的后端集合与当前集合不同时，回调函数将被调用，
    /// 以便调用者可以相应地更新选择器。
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

            // 确保 `callback()` 首先执行是很重要的，因为计算选择器后端可能会很耗时。
            // 例如，如果调用者检查 `backends` 以查看是否有可用的后端，
            // 如果选择器尚未准备好，他们可能会遇到误报。
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

    /// 检查指定的后端服务是否可以接收流量
    ///
    /// 在以下情况下返回 true:
    /// - 后端服务既健康又启用时
    /// - 未配置健康检查但后端服务已启用时
    ///
    /// 当配置了健康检查时，对于任何未知的后端服务都将返回 false
    pub fn ready(&self, backend: &Backend) -> bool {
        self.health
            .load()
            .get(&backend.key())
            // Racing: return `None` when this function is called between the
            // backend store and the health store
            .map_or(self.health_check.is_none(), |h| h.ready())
    }

    /// 手动设置一个 [Backend] 是否可以接收流量。
    ///
    /// 此方法不会覆盖后端的健康状态。它的目的是在后端仍然健康时，停止其接受流量。
    ///
    /// 如果给定的后端不存在于服务发现中，此方法将无操作。
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
