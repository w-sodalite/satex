use crate::make::MakeRouteService;
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::{Configurable, Make};
use serde::{Deserialize, Serialize};

pub use tower_http::services::ServeDir;

#[derive(Debug, Clone, Default, Deserialize, Serialize, Configurable)]
#[configurable(companion = "MakeServeDirRouteService")]
struct Config {
    path: String,
}

#[derive(Debug, Copy, Clone, Make)]
#[make(name = "ServeDir")]
pub struct MakeServeDirRouteService;

impl MakeRouteService for MakeServeDirRouteService {
    type Service = ServeDir;

    fn make(&self, args: Args) -> Result<Self::Service, Error> {
        Config::with_args(args).map(|config| ServeDir::new(config.path))
    }
}
