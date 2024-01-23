use tower_http::limit::RequestBodyLimitLayer;

use satex_core::config::args::Args;
use satex_core::Error;

use crate::make::make_layer;
use crate::make::MakeRouteServiceLayer;

const DEFAULT_REQUEST_BODY_LIMIT: u64 = 10 * 1024;

make_layer! {
    RequestBodyLimit,
    #[serde(deserialize_with = "satex_core::serde::tot::as_u64", default = "Config::default_limit")]
    max: u64,
}

impl Config {
    fn default_limit() -> u64 {
        DEFAULT_REQUEST_BODY_LIMIT
    }
}

fn make(args: Args) -> Result<RequestBodyLimitLayer, Error> {
    Config::try_from(args).map(|config| RequestBodyLimitLayer::new(config.max as usize))
}
