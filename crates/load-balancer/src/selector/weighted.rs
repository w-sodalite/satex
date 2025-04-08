use crate::selector::algorithm::Algorithm;
use crate::selector::{BackendIter, Selector};
use crate::Backend;
use arc_swap::ArcSwap;
use fnv::FnvHasher;
use std::collections::BTreeSet;
use std::sync::Arc;

struct Inner<H> {
    backends: Box<[Backend]>,
    weighted: Box<[u16]>,
    algorithm: H,
}

impl<H: Algorithm> Inner<H> {
    fn build(backends: &BTreeSet<Backend>) -> Self {
        let mut backends = Vec::from_iter(backends.iter().cloned());
        backends.sort_unstable();
        let mut weighted = Vec::with_capacity(backends.len());
        for (index, b) in backends.iter().enumerate() {
            for _ in 0..b.weight {
                weighted.push(index as u16);
            }
        }
        Inner {
            backends: backends.into_boxed_slice(),
            weighted: weighted.into_boxed_slice(),
            algorithm: H::new(),
        }
    }
}

pub struct Weighted<H = FnvHasher>(ArcSwap<Inner<H>>);

impl<H: Algorithm> Weighted<H> {
    pub fn new(backends: &BTreeSet<Backend>) -> Self {
        Self(ArcSwap::new(Arc::new(Inner::build(backends))))
    }
}

impl<H: Algorithm> Selector for Weighted<H> {
    type Iter = WeightedIterator<H>;

    fn update(&self, backends: &BTreeSet<Backend>) {
        self.0.store(Arc::new(Inner::build(backends)))
    }

    fn iter(&self, key: &[u8]) -> Self::Iter {
        let inner = self.0.load_full();
        let index = inner.algorithm.next(key);
        WeightedIterator::new(index, inner)
    }
}

/// An iterator over the backends of a [Weighted] selection.
///
/// See [super::BackendSelection] for more information.
pub struct WeightedIterator<H> {
    // the unbounded index seed
    index: u64,
    inner: Arc<Inner<H>>,
    first: bool,
}

impl<H> WeightedIterator<H> {
    /// Constructs a new [WeightedIterator].
    pub fn new(index: u64, inner: Arc<Inner<H>>) -> Self {
        Self {
            index,
            inner,
            first: true,
        }
    }
}

impl<H: Algorithm> BackendIter for WeightedIterator<H> {
    fn next(&mut self) -> Option<&Backend> {
        if self.inner.backends.is_empty() {
            // short circuit if empty
            return None;
        }

        if self.first {
            // initial hash, select from the weighted list
            self.first = false;
            let len = self.inner.weighted.len();
            let index = self.inner.weighted[self.index as usize % len];
            Some(&self.inner.backends[index as usize])
        } else {
            // fallback, select from the unique list
            // deterministically select the next item
            self.index = self.inner.algorithm.next(&self.index.to_le_bytes());
            let len = self.inner.backends.len();
            Some(&self.inner.backends[self.index as usize % len])
        }
    }
}
