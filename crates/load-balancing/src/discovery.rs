use crate::Backend;
use arc_swap::ArcSwap;
use async_trait::async_trait;
use satex_core::Error;
use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;

#[async_trait]
pub trait Discovery {
    async fn discover(&self) -> Result<(BTreeSet<Backend>, HashMap<u64, bool>), Error>;
}

pub struct Static {
    backends: ArcSwap<BTreeSet<Backend>>,
}

impl Static {
    pub fn new(backends: BTreeSet<Backend>) -> Self {
        Self {
            backends: ArcSwap::new(Arc::new(backends)),
        }
    }
}

#[async_trait]
impl Discovery for Static {
    async fn discover(&self) -> Result<(BTreeSet<Backend>, HashMap<u64, bool>), Error> {
        Ok((BTreeSet::clone(&self.backends.load()), HashMap::new()))
    }
}
