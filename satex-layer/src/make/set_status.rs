use hyper::StatusCode;
use tower_http::set_status::SetStatusLayer;

use satex_core::config::args::Args;
use satex_core::Error;

use crate::__layer;
use crate::make::MakeRouteServiceLayer;

__layer! {
    SetStatus,
    #[serde(with = "http_serde::status_code")]
    status: StatusCode,
}

fn make(args: Args) -> Result<SetStatusLayer, Error> {
    Config::try_from(args).map(|config| SetStatusLayer::new(config.status))
}
