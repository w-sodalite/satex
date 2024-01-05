use satex_core::config::args::Args;
use satex_core::serde::http::{SerdeHeaderName, SerdeHeaderValue};
use satex_core::Error;

use crate::make::set_header::common::{FixedMakeHeaderValue, InsertHeaderMode};
use crate::{MakeRouteServiceLayer, __layer};

type SetResponseHeaderLayer = tower_http::set_header::SetResponseHeaderLayer<FixedMakeHeaderValue>;

__layer! {
    SetResponseHeader,
    name: SerdeHeaderName,
    value: SerdeHeaderValue,
    mode: Option<InsertHeaderMode>,
}

fn make(args: Args) -> Result<SetResponseHeaderLayer, Error> {
    let config = Config::try_from(args)?;
    let make = FixedMakeHeaderValue::new(config.value.into());
    let header_name = config.name.into();
    match config.mode {
        Some(InsertHeaderMode::Append) => Ok(SetResponseHeaderLayer::appending(header_name, make)),
        Some(InsertHeaderMode::IfNotPresent) => {
            Ok(SetResponseHeaderLayer::if_not_present(header_name, make))
        }
        _ => Ok(SetResponseHeaderLayer::overriding(header_name, make)),
    }
}
