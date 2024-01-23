use tower_http::services::ServeDir;

use satex_core::config::args::Args;
use satex_core::Error;

use crate::make::{make_service, MakeRouteService};

type StaticService = ServeDir;

make_service! {
    Static,
    path: String,
}

fn make(args: Args) -> Result<StaticService, Error> {
    Config::try_from(args).map(|config| ServeDir::new(config.path))
}
