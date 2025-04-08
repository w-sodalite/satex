use crate::selector::{BackendIter, Selector};
use crate::Backend;
use arc_swap::ArcSwap;
use pingora_ketama::{Bucket, Continuum};
use std::collections::{BTreeSet, HashMap};
use std::net::SocketAddr;
use std::sync::Arc;

struct Inner {
    ring: Continuum,
    backends: HashMap<SocketAddr, Backend>,
}

impl Inner {
    fn build(backends: &BTreeSet<Backend>) -> Self {
        let buckets = backends
            .iter()
            .map(|b| Bucket::new(b.addr, b.weight as u32))
            .collect::<Vec<_>>();
        let backends = backends.iter().map(|b| (b.addr, b.clone())).collect();
        Self {
            ring: Continuum::new(&buckets),
            backends,
        }
    }
}

/// Weighted Ketama consistent hashing
pub struct KetamaHashing(ArcSwap<Inner>);

impl KetamaHashing {
    pub fn new(backends: &BTreeSet<Backend>) -> Self {
        Self(ArcSwap::new(Arc::new(Inner::build(backends))))
    }
}

impl Selector for KetamaHashing {
    type Iter = OwnedNodeIterator;

    fn update(&self, backends: &BTreeSet<Backend>) {
        self.0.store(Arc::new(Inner::build(backends)))
    }

    fn iter(&self, key: &[u8]) -> Self::Iter {
        let inner = self.0.load_full();
        OwnedNodeIterator {
            idx: inner.ring.node_idx(key),
            inner,
        }
    }
}

/// Iterator over a Continuum
pub struct OwnedNodeIterator {
    idx: usize,
    inner: Arc<Inner>,
}

impl BackendIter for OwnedNodeIterator {
    fn next(&mut self) -> Option<&Backend> {
        self.inner
            .ring
            .get_addr(&mut self.idx)
            .and_then(|addr| self.inner.backends.get(addr))
    }
}
