use crate::make::MakeRouteLayer;
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::{Configurable, Make};
use serde::Deserialize;
use tower::limit::ConcurrencyLimitLayer;

#[derive(Deserialize, Configurable)]
#[configurable(companion = "MakeConcurrencyLimitRouteLayer")]
struct Config {
    max: usize,
}

#[derive(Debug, Clone, Make)]
#[make(name = "ConcurrencyLimit")]
pub struct MakeConcurrencyLimitRouteLayer;

impl MakeRouteLayer for MakeConcurrencyLimitRouteLayer {
    type Layer = ConcurrencyLimitLayer;

    fn make(&self, args: Args) -> Result<Self::Layer, Error> {
        Config::with_args(args).map(|config| ConcurrencyLimitLayer::new(config.max))
    }
}
