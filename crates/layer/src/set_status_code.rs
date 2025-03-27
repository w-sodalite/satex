use crate::make::MakeRouteLayer;
use http::StatusCode;
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::make;
use tower_http::set_status::SetStatusLayer;

#[make(kind = SetStatusCode)]
struct MakeSetStatusCodeRouteLayer {
    status: u16,
}

impl MakeRouteLayer for MakeSetStatusCodeRouteLayer {
    type Layer = SetStatusLayer;

    fn make(&self, args: Args) -> Result<Self::Layer, Error> {
        Config::with_args(args)
            .and_then(|config| StatusCode::from_u16(config.status).map_err(Error::new))
            .map(SetStatusLayer::new)
    }
}
