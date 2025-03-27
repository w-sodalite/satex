use crate::make::MakeRouteLayer;
use crate::set_prefix::layer::SetPrefixLayer;
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::make;

#[make(kind = SetPrefix)]
pub struct MakeSetPrefixRouteLayer {
    prefix: String,
}

impl MakeRouteLayer for MakeSetPrefixRouteLayer {
    type Layer = SetPrefixLayer;

    fn make(&self, args: Args) -> Result<Self::Layer, Error> {
        Config::with_args(args).map(|config| SetPrefixLayer::new(config.prefix))
    }
}
