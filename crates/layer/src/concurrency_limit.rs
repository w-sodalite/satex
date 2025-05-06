#![doc = include_str!("../docs/concurrency_limit.md")]

use crate::make::MakeRouteLayer;
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::make;
use tower::limit::ConcurrencyLimitLayer;

#[make(kind = ConcurrencyLimit)]
struct MakeConcurrencyLimitRouteLayer {
    max: usize,
}

impl MakeRouteLayer for MakeConcurrencyLimitRouteLayer {
    type Layer = ConcurrencyLimitLayer;

    fn make(&self, args: Args) -> Result<Self::Layer, Error> {
        Config::with_args(args).map(|config| ConcurrencyLimitLayer::new(config.max))
    }
}
