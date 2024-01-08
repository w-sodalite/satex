use satex_core::config::args::Args;
use satex_core::Error;

use crate::make::proxy::ProxyService;
use crate::{MakeRouteService, __make_service};

__make_service! {
    Proxy,
    uri: String,
}

fn make(args: Args) -> Result<ProxyService, Error> {
    Config::try_from(args).map(|config| ProxyService::new(config.uri))
}
