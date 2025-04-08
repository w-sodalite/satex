use crate::resolver::{ArcLoadBalancerResolver, LoadBalancerResolver};
use satex_core::Error;
use satex_core::component::Args;
use satex_core::make::Make;
use std::sync::Arc;

pub trait MakeLoadBalancerResolver: Make {
    type Resolver: LoadBalancerResolver;
    fn make(&self, args: Args) -> Result<Self::Resolver, Error>;
}

#[derive(Clone)]
pub struct ArcMakeLoadBalancerResolver(
    Arc<dyn MakeLoadBalancerResolver<Resolver = ArcLoadBalancerResolver> + Send + Sync>,
);

impl ArcMakeLoadBalancerResolver {
    pub fn new<M>(make: M) -> Self
    where
        M: MakeLoadBalancerResolver + Send + Sync + 'static,
        M::Resolver: Send + Sync + 'static,
    {
        Self(Arc::new(Map(make)))
    }
}

struct Map<M>(M);

impl<M> Make for Map<M>
where
    M: MakeLoadBalancerResolver,
    M::Resolver: 'static + Send + Sync,
{
    fn name(&self) -> &'static str {
        self.0.name()
    }
}

impl<M> MakeLoadBalancerResolver for Map<M>
where
    M: MakeLoadBalancerResolver,
    M::Resolver: Send + Sync + 'static,
{
    type Resolver = ArcLoadBalancerResolver;

    fn make(&self, args: Args) -> Result<Self::Resolver, Error> {
        self.0.make(args).map(ArcLoadBalancerResolver::new)
    }
}

impl Make for ArcMakeLoadBalancerResolver {
    fn name(&self) -> &'static str {
        self.0.name()
    }
}

impl MakeLoadBalancerResolver for ArcMakeLoadBalancerResolver {
    type Resolver = ArcLoadBalancerResolver;

    fn make(&self, args: Args) -> Result<Self::Resolver, Error> {
        self.0.make(args)
    }
}
