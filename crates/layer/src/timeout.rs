use crate::make::MakeRouteLayer;
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::make;
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;

#[make(kind = Timeout)]
struct MakeTimeoutRouteLayer {
    timeout: u64,
}

impl MakeRouteLayer for MakeTimeoutRouteLayer {
    type Layer = TimeoutLayer;

    fn make(&self, args: Args) -> Result<Self::Layer, Error> {
        Config::with_args(args)
            .map(|config| TimeoutLayer::new(Duration::from_millis(config.timeout)))
    }
}
