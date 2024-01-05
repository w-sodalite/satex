use satex_core::config::args::Args;
use satex_core::Error;

use crate::make::proxy::ProxyService;
use crate::{MakeRouteService, __service};

__service! {
    Proxy,
    uri: String,
}

fn make(args: Args) -> Result<ProxyService, Error> {
    Config::try_from(args).map(|config| ProxyService::new(config.uri))
}
