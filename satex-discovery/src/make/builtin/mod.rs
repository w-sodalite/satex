use std::collections::HashMap;

use async_trait::async_trait;
use tracing::debug;

pub use make::MakeBuiltinDiscovery;
use satex_core::endpoint::Endpoint;
use satex_core::essential::Essential;
use satex_core::Error;

use crate::lb::{LoadBalance, NamedLoadBalance};
use crate::selector::Selector;
use crate::ServerDiscovery;

mod make;

pub struct BuiltinDiscovery {
    items: HashMap<String, (Selector, NamedLoadBalance)>,
}

impl BuiltinDiscovery {
    ///
    /// 创建内置的服务发现对象
    ///
    /// # Arguments
    ///
    /// * `items`: 所有服务对应的选择器和负载均衡策略
    ///
    /// returns: BuiltinDiscovery
    ///
    pub fn new(items: HashMap<String, (Selector, NamedLoadBalance)>) -> Self {
        Self { items }
    }
}

#[async_trait]
impl ServerDiscovery for BuiltinDiscovery {
    async fn resolve(
        &self,
        essential: &Essential,
        server: &str,
    ) -> Result<Option<Endpoint>, Error> {
        match self.items.get(server) {
            Some((selector, lb)) => {
                let endpoints = selector.select().await?;
                match endpoints.len() {
                    0 => Ok(None),
                    1 => Ok(endpoints.into_iter().next().map(Endpoint::from)),
                    _ => lb.choose(essential, endpoints).await,
                }
            }
            None => {
                debug!("Cannot find valid selector for server: {}", server);
                Ok(None)
            }
        }
    }
}
