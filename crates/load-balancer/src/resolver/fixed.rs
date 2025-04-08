use crate::discovery::FixedDiscovery;
use crate::resolver::LoadBalancerResolver;
use crate::resolver::make::MakeLoadBalancerResolver;
use crate::selector::{BoxSelector, Consistent, Random, RoundRobin};
use crate::{Backend, Backends, LoadBalancer};
use satex_core::Error;
use satex_core::component::{Args, Configurable};
use satex_macro::make;
use serde::Deserialize;
use std::collections::{BTreeSet, HashMap};
use std::net::SocketAddr;
use std::sync::Arc;

pub struct FixedLoadBalancerResolver {
    load_balancers: HashMap<String, Arc<LoadBalancer>>,
}

impl LoadBalancerResolver for FixedLoadBalancerResolver {
    fn find(&self, name: &str) -> Option<Arc<LoadBalancer>> {
        self.load_balancers.get(name).cloned()
    }
}

#[derive(Deserialize)]
struct Upstream {
    name: String,
    policy: Policy,
    addrs: Vec<SocketAddr>,
}

#[derive(Deserialize)]
enum Policy {
    RoundRobin,
    Random,
    Consistent,
}

#[make(kind = "Fixed")]
pub struct MakeFixedLoadBalancerResolver {
    upstreams: Vec<Upstream>,
}

impl MakeLoadBalancerResolver for MakeFixedLoadBalancerResolver {
    type Resolver = FixedLoadBalancerResolver;

    fn make(&self, args: Args) -> Result<Self::Resolver, Error> {
        let config = Config::with_args(args)?;
        let load_balancers = config
            .upstreams
            .into_iter()
            .map(|upstream| {
                let backends = upstream
                    .addrs
                    .into_iter()
                    .map(Backend::new)
                    .collect::<BTreeSet<_>>();

                let selector = match upstream.policy {
                    Policy::RoundRobin => BoxSelector::new(RoundRobin::new(&backends)),
                    Policy::Random => BoxSelector::new(Random::new(&backends)),
                    Policy::Consistent => BoxSelector::new(Consistent::new(&backends)),
                };

                let backends = Backends::new(FixedDiscovery::new(backends));
                (
                    upstream.name,
                    Arc::new(LoadBalancer::new(backends, selector)),
                )
            })
            .collect::<HashMap<_, _>>();
        Ok(FixedLoadBalancerResolver { load_balancers })
    }
}
