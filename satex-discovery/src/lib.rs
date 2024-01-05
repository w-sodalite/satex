#![allow(dead_code)]

use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use async_trait::async_trait;

use satex_core::endpoint::Endpoint;
use satex_core::essential::Essential;
use satex_core::{export_make, Error};

pub mod lb;
mod make;
mod registry;
mod selector;
export_make!(MakeServerDiscovery);

#[async_trait]
pub trait ServerDiscovery {
    async fn resolve(&self, essential: &Essential, server: &str)
        -> Result<Option<Endpoint>, Error>;
}

#[derive(Clone)]
pub struct NamedServerDiscovery {
    name: &'static str,
    inner: Arc<dyn ServerDiscovery + Send + Sync + 'static>,
}

impl Debug for NamedServerDiscovery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServerDiscovery")
            .field("name", &self.name)
            .finish()
    }
}

impl NamedServerDiscovery {
    pub fn new<D: ServerDiscovery + Send + Sync + 'static>(
        name: &'static str,
        discovery: D,
    ) -> Self {
        Self {
            name,
            inner: Arc::new(discovery),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn composite(discoveries: Vec<NamedServerDiscovery>) -> NamedServerDiscovery {
        NamedServerDiscovery::new("Composite", CompositeServerDiscovery::new(discoveries))
    }
}

#[async_trait]
impl ServerDiscovery for NamedServerDiscovery {
    async fn resolve(
        &self,
        essential: &Essential,
        server: &str,
    ) -> Result<Option<Endpoint>, Error> {
        self.inner.resolve(essential, server).await
    }
}

struct CompositeServerDiscovery {
    discoveries: Vec<NamedServerDiscovery>,
}

impl CompositeServerDiscovery {
    pub fn new(discoveries: Vec<NamedServerDiscovery>) -> Self {
        Self { discoveries }
    }
}

#[async_trait]
impl ServerDiscovery for CompositeServerDiscovery {
    async fn resolve(
        &self,
        essential: &Essential,
        server: &str,
    ) -> Result<Option<Endpoint>, Error> {
        for discovery in self.discoveries.iter() {
            match discovery.resolve(essential, server).await {
                Ok(None) => continue,
                Ok(Some(endpoint)) => return Ok(Some(endpoint)),
                Err(e) => return Err(e),
            }
        }
        Ok(None)
    }
}
