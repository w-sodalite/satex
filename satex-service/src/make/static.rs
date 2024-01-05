use tower_http::services::ServeDir;

use satex_core::config::args::Args;
use satex_core::Error;

use crate::__service;
use crate::make::MakeRouteService;

type StaticService = ServeDir;

__service! {
    Static,
    directory: String,
}

fn make(args: Args) -> Result<StaticService, Error> {
    Config::try_from(args).map(|config| ServeDir::new(config.directory))
}
