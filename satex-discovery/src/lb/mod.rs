use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use async_trait::async_trait;

use satex_core::endpoint::Endpoint;
use satex_core::essential::Essential;
use satex_core::{export_make, Error};

use crate::lb::make::sequential::SequentialLoadBalance;
use crate::selector::IndexedEndpoint;

mod make;
mod registry;
export_make!(MakeLoadBalance);

#[derive(Clone)]
pub struct Context<'a> {
    pub essential: &'a Essential,
    pub endpoints: Vec<IndexedEndpoint>,
}

impl<'a> Context<'a> {
    pub fn new(essential: &'a Essential, endpoints: Vec<IndexedEndpoint>) -> Self {
        Self {
            essential,
            endpoints,
        }
    }
}

#[async_trait]
pub trait LoadBalance {
    async fn choose<'a>(&self, context: Context<'a>) -> Result<Option<Endpoint>, Error>;
}

#[derive(Clone)]
pub struct NamedLoadBalance {
    name: &'static str,
    inner: Arc<dyn LoadBalance + Send + Sync + 'static>,
}

impl Debug for NamedLoadBalance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoadBalance")
            .field("name", &self.name)
            .finish()
    }
}

impl NamedLoadBalance {
    pub fn new<LB: LoadBalance + Send + Sync + 'static>(name: &'static str, lb: LB) -> Self {
        Self {
            name,
            inner: Arc::new(lb),
        }
    }
}

impl Default for NamedLoadBalance {
    fn default() -> Self {
        NamedLoadBalance::new("Sequential", SequentialLoadBalance::default())
    }
}

#[async_trait]
impl LoadBalance for NamedLoadBalance {
    async fn choose<'a>(&self, context: Context<'a>) -> Result<Option<Endpoint>, Error> {
        self.inner.choose(context).await
    }
}
