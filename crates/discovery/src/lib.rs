use crate::backend::Backend;
use crate::health_check::{ArcHealthCheck, HealthCheck, TcpHealthCheck};
use futures::Stream;
use satex_core::Error;
use std::collections::BTreeSet;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

mod backend;
mod health_check;

pub type ArcDiscovery = Arc<dyn Discovery + Send + Sync>;

pub enum Change {
    Insert(Backend),
    Remove(u64),
}

pub trait Discovery {
    /// Yields the next discovery change set.
    fn poll_discover(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Change, Error>>>;
}


#[derive(Clone)]
pub struct StaticDiscovery {
    backends: BTreeSet<Backend>,
    health_check: ArcHealthCheck,
}

impl StaticDiscovery {
    pub fn new(backends: BTreeSet<Backend>) -> Self {
        Self {
            backends,
            health_check: Arc::new(TcpHealthCheck::default()),
        }
    }

    pub fn with_health_check(self, health_check: impl HealthCheck + Send + Sync + 'static) -> Self {
        Self {
            backends: self.backends,
            health_check: Arc::new(health_check),
        }
    }
}

impl Discovery for StaticDiscovery {
    fn poll_discover(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Change, Error>>> {
        Poll::Ready(None)
    }
}
