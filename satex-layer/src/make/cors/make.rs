use std::time::Duration;

use tower_http::cors::CorsLayer;

use satex_core::config::args::Args;
use satex_core::serde::http::{SerdeHeaderName, SerdeHeaderValue, SerdeMethod};
use satex_core::Error;

use crate::make::cors::complex::*;
use crate::{MakeRouteServiceLayer, __make_layer};

__make_layer! {
    Cors,
    allow_credentials: Option<bool>,
    allow_headers: Option<Complex<SerdeHeaderName>>,
    allow_methods: Option<Complex<SerdeMethod>>,
    allow_origin: Option<Complex<SerdeHeaderValue>>,
    allow_private_network: Option<bool>,
    expose_headers: Option<Complex<SerdeHeaderName>>,
    max_age: Option<u64>,
    vary: Option<Complex<SerdeHeaderName>>,
}

macro_rules! set_props {
    ($layer:ident, [$($prop:ident),+ $(,)?]) => {
        $(
            if let Some($prop) = $prop {
                $layer = $layer.$prop($prop);
            }
        )+
    };
}

fn make(args: Args) -> Result<CorsLayer, Error> {
    let Config {
        allow_credentials,
        allow_headers,
        allow_methods,
        allow_origin,
        allow_private_network,
        expose_headers,
        max_age,
        vary,
    } = Config::try_from(args)?;
    let mut layer = CorsLayer::default();
    if let Some(max_age) = max_age {
        layer = layer.max_age(Duration::from_secs(max_age));
    }
    set_props!(
        layer,
        [
            allow_credentials,
            allow_headers,
            allow_methods,
            allow_origin,
            allow_private_network,
            expose_headers,
            vary
        ]
    );
    Ok(layer)
}

#[cfg(test)]
mod test {

    #[test]
    fn test_layer() {}
}
