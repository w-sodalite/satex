use crate::make::MakeRouteService;
use crate::proxy::client::{Client, ClientConfig};
use crate::proxy::service::ProxyRouteService;
use http::Uri;
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::make;
use std::str::FromStr;

#[make(kind = Proxy)]
struct MakeProxyRouteService {
    uri: String,
    #[serde(default)]
    client: ClientConfig,
}

impl MakeRouteService for MakeProxyRouteService {
    type Service = ProxyRouteService;

    fn make(&self, args: Args) -> Result<Self::Service, Error> {
        Config::with_args(args).and_then(|config| {
            Uri::from_str(&config.uri)
                .map_err(Error::new)
                .map(|uri| ProxyRouteService::new(uri, Client::from(config.client)))
        })
    }
}
