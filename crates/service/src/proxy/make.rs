use crate::make::MakeRouteService;
use crate::proxy::client::{Client, ClientConfig};
use crate::proxy::service::ProxyRouteService;
use http::Uri;
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::{Configurable, Make};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize, Configurable)]
#[configurable(companion = "MakeProxyRouteService")]
struct Config {
    uri: String,
    #[serde(default)]
    client: ClientConfig,
}

#[derive(Debug, Clone, Make)]
#[make(name = "Proxy")]
pub struct MakeProxyRouteService;

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
