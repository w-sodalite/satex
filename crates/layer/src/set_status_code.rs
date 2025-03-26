use crate::make::MakeRouteLayer;
use http::StatusCode;
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::{Configurable, Make};
use serde::Deserialize;
use tower_http::set_status::SetStatusLayer;

#[derive(Deserialize, Configurable)]
#[configurable(companion = "MakeSetResponseStatusCodeRouteLayer")]
struct Config {
    status: u16,
}

#[derive(Debug, Clone, Copy, Default, Make)]
#[make(name = "SetResponseStatusCode")]
pub struct MakeSetResponseStatusCodeRouteLayer;

impl MakeRouteLayer for MakeSetResponseStatusCodeRouteLayer {
    type Layer = SetStatusLayer;

    fn make(&self, args: Args) -> Result<Self::Layer, Error> {
        Config::with_args(args)
            .and_then(|config| StatusCode::from_u16(config.status).map_err(Error::new))
            .map(SetStatusLayer::new)
    }
}
