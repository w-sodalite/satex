use crate::discovery::StaticFixedDiscovery;
use crate::health_check::tcp::TcpHealthCheck;
use crate::resolver::make::MakeLoadBalancerResolver;
use crate::resolver::LoadBalancerResolver;
use crate::selector::{BoxSelector, Consistent, Random, RoundRobin};
use crate::{Backend, Backends, LoadBalancer};
use satex_core::background::background_task;
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::make;
use serde::Deserialize;
use std::collections::{BTreeSet, HashMap};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::spawn;

pub struct StaticLoadBalancerResolver {
    load_balancers: HashMap<String, Arc<LoadBalancer>>,
}

impl LoadBalancerResolver for StaticLoadBalancerResolver {
    fn find(&self, name: &str) -> Option<Arc<LoadBalancer>> {
        self.load_balancers.get(name).cloned()
    }
}

#[derive(Deserialize)]
struct Upstream {
    name: String,
    #[serde(default)]
    policy: Policy,
    addrs: Vec<SocketAddr>,
    #[serde(default, rename = "health-check")]
    health_check: HealthCheck,
}

#[derive(Deserialize, Default)]
enum Policy {
    #[default]
    RoundRobin,
    Random,
    Consistent,
}

#[derive(Deserialize)]
struct HealthCheck {
    enabled: bool,
}

impl Default for HealthCheck {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[make(kind = "Static", shortcut_mode = Sequence)]
pub struct MakeStaticLoadBalancerResolver {
    upstreams: Vec<Upstream>,
}

impl MakeLoadBalancerResolver for MakeStaticLoadBalancerResolver {
    type Resolver = StaticLoadBalancerResolver;

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

                let backends = Backends::new(StaticFixedDiscovery::new(backends))
                    .with_health_check(TcpHealthCheck::default());
                let load_balancer = Arc::new(LoadBalancer::new(backends, selector));

                if upstream.health_check.enabled {
                    let task = background_task(
                        format!("LoadBalancer - {}", upstream.name),
                        load_balancer.clone(),
                    );
                    spawn(task);
                }

                (upstream.name, load_balancer)
            })
            .collect::<HashMap<_, _>>();
        Ok(StaticLoadBalancerResolver { load_balancers })
    }
}
