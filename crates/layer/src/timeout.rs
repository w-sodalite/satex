use crate::make::MakeRouteLayer;
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::{Configurable, Make};
use serde::Deserialize;
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;

#[derive(Deserialize, Configurable)]
#[configurable(companion = "")]
struct Config {
    timeout: u64,
}

#[derive(Debug, Clone, Copy, Make)]
#[make(name = "Timeout")]
pub struct MakeTimeoutRouteLayer;

impl MakeRouteLayer for MakeTimeoutRouteLayer {
    type Layer = TimeoutLayer;

    fn make(&self, args: Args) -> Result<Self::Layer, Error> {
        Config::with_args(args)
            .map(|config| TimeoutLayer::new(Duration::from_millis(config.timeout)))
    }
}
