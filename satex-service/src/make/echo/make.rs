use satex_core::config::args::Args;
use satex_core::Error;

use crate::make::echo::EchoService;
use crate::make::make_service;
use crate::MakeRouteService;

make_service!(Echo);

fn make(_: Args) -> Result<EchoService, Error> {
    Ok(EchoService)
}
