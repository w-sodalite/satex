use crate::health_check::HealthCheck;
use crate::selector::{BoxSelector, BackendIter, Selector};
use crate::{Backend, Backends};
use satex_core::Error;
use std::time::Duration;

pub struct LoadBalancer {
    selector: BoxSelector,
    pub(crate) backends: Backends,
    pub(crate) update_frequency: Option<Duration>,
    pub(crate) health_check_frequency: Option<Duration>,
    pub(crate) health_check_parallel: bool,
}

impl LoadBalancer {
    pub fn new<S>(backends: Backends, selector: S) -> Self
    where
        S: Selector + Send + Sync + 'static,
        S::Iter: Send + Sync + 'static,
    {
        let selector = BoxSelector::new(selector);
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
