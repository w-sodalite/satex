use crate::registry::push;
use satex_load_balancer::resolver::MakeStaticLoadBalancerResolver;
use satex_load_balancer::resolver::{ArcMakeLoadBalancerResolver, MakeLoadBalancerResolver};
use std::collections::HashMap;

#[derive(Clone)]
pub struct MakeLoadBalancerResolverRegistry(HashMap<&'static str, ArcMakeLoadBalancerResolver>);

impl MakeLoadBalancerResolverRegistry {
    pub fn without_default() -> Self {
        Self(HashMap::new())
    }

    pub fn with_default() -> Self {
        let mut registry = Self::without_default();
        push! {
            registry,
            MakeStaticLoadBalancerResolver
        }
        registry
    }

    pub fn push<M>(&mut self, make: M)
    where
        M: MakeLoadBalancerResolver + Send + Sync + 'static,
        M::Resolver: Send + Sync + 'static,
    {
        self.0
            .insert(make.name(), ArcMakeLoadBalancerResolver::new(make));
    }

    pub fn get(&self, name: &str) -> Option<ArcMakeLoadBalancerResolver> {
        self.0.get(name).cloned()
    }
}
