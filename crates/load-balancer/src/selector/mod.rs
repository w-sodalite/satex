mod algorithm;
mod consistent;
mod weighted;

use crate::selector::weighted::Weighted;
use crate::Backend;
use std::collections::BTreeSet;

/// Random selection on weighted backends
pub type Random = Weighted<algorithm::Random>;

/// Round-robin selection on weighted backends
pub type RoundRobin = Weighted<algorithm::RoundRobin>;

/// Consistent Ketama hashing on weighted backends
pub type Consistent = consistent::KetamaHashing;

pub trait BackendIter {
    fn next(&mut self) -> Option<&Backend>;
}

pub struct BoxBackendIter(Box<dyn BackendIter + Send + Sync>);

impl BackendIter for BoxBackendIter {
    fn next(&mut self) -> Option<&Backend> {
        self.0.next()
    }
}

pub trait Selector {
    type Iter: BackendIter;

    fn update(&self, backends: &BTreeSet<Backend>);

    fn iter(&self, key: &[u8]) -> Self::Iter;
}

pub(crate) struct Map<S>(S);

impl<S> Map<S> {
    pub fn new(selector: S) -> Self {
        Self(selector)
    }
}

impl<S> Selector for Map<S>
where
    S: Selector + Send + Sync,
    S::Iter: Send + Sync + 'static,
{
    type Iter = BoxBackendIter;

    fn update(&self, backends: &BTreeSet<Backend>) {
        self.0.update(backends);
    }

    fn iter(&self, key: &[u8]) -> Self::Iter {
        BoxBackendIter(Box::new(self.0.iter(key)))
    }
}

pub struct BoxSelector(Box<dyn Selector<Iter = BoxBackendIter> + Send + Sync>);

impl BoxSelector {
    pub fn new<S>(selector: S) -> Self
    where
        S: Selector + Send + Sync + 'static,
        S::Iter: Send + Sync + 'static,
    {
        Self(Box::new(Map::new(selector)))
    }
}

impl Selector for BoxSelector {
    type Iter = BoxBackendIter;

    fn update(&self, backends: &BTreeSet<Backend>) {
        self.0.update(backends)
    }

    fn iter(&self, key: &[u8]) -> Self::Iter {
        self.0.iter(key)
    }
}
