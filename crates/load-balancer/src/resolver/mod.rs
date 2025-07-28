mod make;
mod r#static;

pub use make::*;
pub use r#static::*;

use crate::LoadBalancer;
use std::sync::Arc;

pub trait LoadBalancerResolver {
    fn find(&self, name: &str) -> Option<Arc<LoadBalancer>>;
}

#[derive(Clone)]
pub struct ArcLoadBalancerResolver(Arc<dyn LoadBalancerResolver + Send + Sync>);

impl ArcLoadBalancerResolver {
    pub fn new<R>(resolver: R) -> Self
    where
        R: LoadBalancerResolver + Send + Sync + 'static,
    {
        Self(Arc::new(resolver))
    }
}

impl LoadBalancerResolver for ArcLoadBalancerResolver {
    fn find(&self, name: &str) -> Option<Arc<LoadBalancer>> {
        self.0.find(name)
    }
}

#[derive(Default)]
pub struct CompositeLoadBalancerResolver(Vec<ArcLoadBalancerResolver>);

impl CompositeLoadBalancerResolver {
    pub fn push<R>(mut self, resolver: R) -> Self
    where
        R: LoadBalancerResolver + Send + Sync + 'static,
    {
        self.0.push(ArcLoadBalancerResolver::new(resolver));
        self
    }
}

impl LoadBalancerResolver for CompositeLoadBalancerResolver {
    fn find(&self, name: &str) -> Option<Arc<LoadBalancer>> {
        for resolver in self.0.iter() {
            if let Some(load_balancer) = resolver.find(name) {
                return Some(load_balancer);
            }
        }
        None
    }
}
