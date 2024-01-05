use satex_core::config::args::Args;
use satex_core::Error;

use crate::make::path_strip::layer::PathStripLayer;
use crate::{MakeRouteServiceLayer, __layer};

__layer! {
    PathStrip,
    #[serde(deserialize_with = "satex_core::serde::tot::as_u64")]
    level: u64,
}

fn make(args: Args) -> Result<PathStripLayer, Error> {
    Config::try_from(args).map(|config| PathStripLayer::new(config.level as usize))
}
