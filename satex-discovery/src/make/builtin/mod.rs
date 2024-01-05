use std::collections::HashMap;

use async_trait::async_trait;
use tracing::debug;

pub use make::MakeBuiltinDiscovery;
use satex_core::endpoint::Endpoint;
use satex_core::essential::Essential;
use satex_core::Error;

use crate::lb::{Context, LoadBalance, NamedLoadBalance};
use crate::selector::Selector;
use crate::ServerDiscovery;

mod make;

pub struct BuiltinDiscovery {
    selectors: HashMap<String, Selector>,
    lbs: HashMap<String, NamedLoadBalance>,
}

impl BuiltinDiscovery {
    pub fn new(
        selectors: HashMap<String, Selector>,
        lbs: HashMap<String, NamedLoadBalance>,
    ) -> Self {
        Self { selectors, lbs }
    }
}

#[async_trait]
impl ServerDiscovery for BuiltinDiscovery {
    async fn resolve(
        &self,
        essential: &Essential,
        server: &str,
    ) -> Result<Option<Endpoint>, Error> {
        match self.selectors.get(server) {
            Some(selector) => {
                let endpoints = selector.select().await?;
                match endpoints.len() {
                    0 => Ok(None),
                    1 => Ok(endpoints.into_iter().next().map(Endpoint::from)),
                    _ => match self.lbs.get(server) {
                        Some(lb) => lb.choose(Context::new(essential, endpoints)).await,
                        None => Ok(endpoints.into_iter().next().map(Endpoint::from)),
                    },
                }
            }
            None => {
                debug!("Cannot find valid selector for server: {}", server);
                Ok(None)
            }
        }
    }
}
