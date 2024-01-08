use std::time::Duration;

use satex_core::config::args::Args;
use satex_core::Error;

use crate::make::rate_limit::layer::RateLimitLayer;
use crate::make::rate_limit::Policy;
use crate::{MakeRouteServiceLayer, __make_layer};

__make_layer! {
    RateLimit,

    policy: Policy,

    #[serde(deserialize_with = "satex_core::serde::tot::as_u64")]
    max: u64,

    #[serde(deserialize_with = "satex_core::serde::tot::as_u64")]
    refill: u64,

    #[serde(
        default = "Config::default_interval",
        deserialize_with = "satex_core::serde::tot::as_u64"
    )]
    interval: u64,

    #[serde(
        default = "Config::default_fair",
        deserialize_with = "satex_core::serde::tot::as_bool"
    )]
    fair: bool,
}

impl Config {
    fn default_interval() -> u64 {
        1
    }

    fn default_fair() -> bool {
        false
    }
}

fn make(args: Args) -> Result<RateLimitLayer, Error> {
    let config = Config::try_from(args)?;
    Ok(RateLimitLayer::new(
        config.policy,
        config.max as usize,
        config.refill as usize,
        Duration::from_secs(config.interval),
        config.fair,
    ))
}
