use crate::make::MakeRouteLayer;
use crate::strip_prefix::layer::StripPrefixRouteLayer;
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::{Configurable, Make};
use serde::Deserialize;

#[derive(Deserialize, Configurable)]
#[configurable(companion = "MakeStripPrefixRouteLayer")]
struct Config {
    level: usize,
}

#[derive(Debug, Clone, Copy, Default, Make)]
#[make(name = "StripPrefix")]
pub struct MakeStripPrefixRouteLayer;

impl MakeRouteLayer for MakeStripPrefixRouteLayer {
    type Layer = StripPrefixRouteLayer;

    fn make(&self, args: Args) -> Result<Self::Layer, Error> {
        Config::with_args(args).map(|config| StripPrefixRouteLayer::new(config.level))
    }
}
