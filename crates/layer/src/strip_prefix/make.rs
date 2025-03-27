use crate::make::MakeRouteLayer;
use crate::strip_prefix::layer::StripPrefixRouteLayer;
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::make;

#[make(kind = StripPrefix)]
pub struct MakeStripPrefixRouteLayer {
    level: usize,
}

impl MakeRouteLayer for MakeStripPrefixRouteLayer {
    type Layer = StripPrefixRouteLayer;

    fn make(&self, args: Args) -> Result<Self::Layer, Error> {
        Config::with_args(args).map(|config| StripPrefixRouteLayer::new(config.level))
    }
}
