use satex_core::config::args::Args;
use satex_core::Error;

use crate::make::echo::EchoService;
use crate::{MakeRouteService, __make_service};

__make_service!(Echo);

fn make(_: Args) -> Result<EchoService, Error> {
    Ok(EchoService)
}
