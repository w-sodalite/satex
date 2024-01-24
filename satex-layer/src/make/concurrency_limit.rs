use tower::limit::ConcurrencyLimitLayer;

use satex_core::config::args::Args;
use satex_core::Error;

use crate::make::make_layer;
use crate::MakeRouteServiceLayer;

make_layer! {
    ConcurrencyLimit,
    #[serde(deserialize_with = "satex_core::serde::tot::as_u64")]
    max: u64,
}

fn make(args: Args) -> Result<ConcurrencyLimitLayer, Error> {
    Config::try_from(args).map(|config| ConcurrencyLimitLayer::new(config.max as usize))
}
