use crate::make::MakeRouteService;
use crate::proxy::client::{Client, ClientConfig};
use crate::proxy::service::ProxyRouteService;
use http::Extensions;
use satex_core::Error;
use satex_core::component::{Args, Configurable};
use satex_core::digest::DefaultDigester;
use satex_core::util::remove_end_sep;
use satex_load_balancer::LoadBalancer;
use satex_load_balancer::resolver::{ArcLoadBalancerResolver, LoadBalancerResolver};
use satex_macro::make;
use std::str::FromStr;
use std::sync::Arc;
use url::Url;

#[make(kind = Proxy)]
struct MakeProxyRouteService {
    uri: String,
    #[serde(default)]
    client: ClientConfig,
}

impl MakeRouteService for MakeProxyRouteService {
    type Service = ProxyRouteService<DefaultDigester>;

    fn make(&self, args: Args, extensions: &Extensions) -> Result<Self::Service, Error> {
        Config::with_args(args).and_then(|config| {
            Url::from_str(remove_end_sep(&config.uri))
                .map_err(Error::new)
                .map(|url| {
                    let load_balancer = extensions
                        .get::<ArcLoadBalancerResolver>()
                        .and_then(|registry| get_load_balancer(registry, &url));

                    ProxyRouteService::new(
                        url,
                        Client::from(config.client),
                        DefaultDigester,
                        load_balancer,
                    )
                })
        })
    }
}

fn get_load_balancer(resolver: &ArcLoadBalancerResolver, url: &Url) -> Option<Arc<LoadBalancer>> {
    if let Some(load_balancer) = resolver.find(url.host_str().unwrap_or_default()) {
        return Some(load_balancer);
    }
    None
}
