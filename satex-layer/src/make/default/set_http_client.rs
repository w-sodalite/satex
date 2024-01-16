use std::time::Duration;

use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use tower_http::add_extension::AddExtensionLayer;

use satex_core::config::Config;
use satex_core::http::Body;
use satex_core::{satex_error, Error};

use crate::make::default::MakeDefaultRouteServiceLayer;

type HttpClient = Client<HttpsConnector<HttpConnector>, Body>;

#[derive(Default)]
pub struct MakeSetHttpClientLayer;

impl MakeDefaultRouteServiceLayer for MakeSetHttpClientLayer {
    type Layer = AddExtensionLayer<HttpClient>;

    fn name(&self) -> &'static str {
        "SetHttpClient"
    }

    fn make(&self, config: &Config) -> Result<Self::Layer, Error> {
        let config = config.client();
        let connector = HttpsConnectorBuilder::default()
            .with_native_roots()
            .map(|builder| builder.https_or_http())
            .map(|builder| builder.enable_all_versions().build())
            .map_err(|e| satex_error!(e))?;
        Ok(AddExtensionLayer::new(
            Client::builder(TokioExecutor::new())
                .pool_max_idle_per_host(config.pool_max_idle_per_host())
                .pool_idle_timeout(Duration::from_secs(config.pool_idle_timeout_secs()))
                .retry_canceled_requests(config.retry_canceled_requests())
                .set_host(config.set_host())
                .build(connector),
        ))
    }
}
