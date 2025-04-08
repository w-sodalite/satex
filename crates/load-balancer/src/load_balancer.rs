use crate::health_check::HealthCheck;
use crate::selector::{BackendIter, BoxSelector, Selector};
use crate::{Backend, Backends};
use satex_core::Error;
use std::time::Duration;

/// 负载均衡器
pub struct LoadBalancer {
    selector: BoxSelector,
    pub(crate) backends: Backends,
    pub(crate) update_frequency: Option<Duration>,
    pub(crate) health_check_frequency: Option<Duration>,
    pub(crate) health_check_parallel: bool,
}

impl LoadBalancer {
    /// 创建负载均衡器实例
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

    /// 设置健康检查
    pub fn with_health_check(
        mut self,
        health_check: impl HealthCheck + Send + Sync + 'static,
    ) -> Self {
        self.backends = self.backends.with_health_check(health_check);
        self
    }

    /// 设置更新频率
    pub fn with_update_frequency(mut self, update_frequency: Duration) -> Self {
        self.update_frequency = Some(update_frequency);
        self
    }

    /// 设置健康检查的频率
    pub fn with_health_check_frequency(mut self, health_check_frequency: Duration) -> Self {
        self.health_check_frequency = Some(health_check_frequency);
        self
    }

    pub fn with_health_check_parallel(mut self, health_check_parallel: bool) -> Self {
        self.health_check_parallel = health_check_parallel;
        self
    }

    /// 运行服务发现并更新选择算法。
    ///
    /// 如果这个 [LoadBalancer] 实例作为后台服务运行，此函数将每隔 `update_frequency` 被调用一次。
    pub async fn update(&self) -> Result<(), Error> {
        self.backends
            .update(|backends| self.selector.update(&backends))
            .await
    }

    /// 根据选择算法和健康检查结果返回第一个健康的 [Backend]。
    ///
    /// `key` 用于基于哈希的选择，如果选择是随机或轮询，则忽略此参数。
    ///
    /// `max_iterations` 用于限制搜索下一个 Backend 的时间。在某些算法中，
    /// 如 Ketama 哈希，搜索下一个后端是线性的，可能需要很多步骤。
    pub fn select(&self, key: &[u8]) -> Option<Backend> {
        self.select_with(key, |_, health| health)
    }

    /// 类似于 [Self::select]，根据选择算法和用户定义的 `accept` 函数返回第一个健康的 [Backend]。
    ///
    /// `accept` 函数接受两个输入：正在选择的后端和该后端的内部健康状态。该函数可以执行一些操作，
    /// 比如忽略内部健康检查或因为之前失败而跳过这个后端。`accept` 函数会被多次调用，遍历后端，
    /// 直到返回 `true`。
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
