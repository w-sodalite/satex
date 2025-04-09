use crate::make::MakeRouteService;
use http::Extensions;
use satex_core::Error;
use satex_core::component::{Args, Configurable};
use satex_macro::make;

pub use tower_http::services::ServeDir;

#[make(kind = ServeDir)]
struct MakeServeDirRouteService {
    path: String,
}

impl MakeRouteService for MakeServeDirRouteService {
    type Service = ServeDir;

    fn make(&self, args: Args, _: &Extensions) -> Result<Self::Service, Error> {
        Config::with_args(args).map(|config| ServeDir::new(config.path))
    }
}
