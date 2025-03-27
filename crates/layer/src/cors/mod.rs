mod value;

use crate::cors::value::Value;
use crate::make::MakeRouteLayer;
use http::{HeaderName, HeaderValue, Method};
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::make;
use std::time::Duration;
use tower_http::cors::CorsLayer;

#[make(kind = "Cors", shortcut_mode = "Unsupported")]
struct MakeCorsRouteLayer {
    max_age_secs: Option<u64>,
    allow_credentials: Option<bool>,
    allow_private_network: Option<bool>,
    #[serde(default)]
    allow_headers: Value<HeaderName>,
    #[serde(default)]
    allow_methods: Value<Method>,
    #[serde(default)]
    allow_origin: Value<HeaderValue>,
    #[serde(default)]
    expose_headers: Value<HeaderName>,
    #[serde(default)]
    vary: Value<HeaderName>,
}

impl MakeRouteLayer for MakeCorsRouteLayer {
    type Layer = CorsLayer;

    fn make(&self, args: Args) -> Result<Self::Layer, Error> {
        let config = Config::with_args(args)?;
        let mut layer = CorsLayer::default()
            .allow_headers(config.allow_headers)
            .allow_origin(config.allow_origin)
            .allow_methods(config.allow_methods)
            .expose_headers(config.expose_headers)
            .vary(config.vary);

        if let Some(allow_credentials) = config.allow_credentials {
            layer = layer.allow_credentials(allow_credentials);
        }
        if let Some(max_age_secs) = config.max_age_secs {
            layer = layer.max_age(Duration::from_secs(max_age_secs));
        }
        if let Some(allow_private_network) = config.allow_private_network {
            layer = layer.allow_private_network(allow_private_network);
        }

        Ok(layer)
    }
}
