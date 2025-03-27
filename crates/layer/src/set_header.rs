pub use request::MakeSetRequestHeaderRouteLayer;
pub use response::MakeSetResponseHeaderRouteLayer;

macro_rules! make_set_header {
    ($name:ident, $make:ident, $layer:ident) => {
        #[make(kind = $name)]
        pub struct $make {
            name: String,
            value: String,
            #[serde(default)]
            policy: SetPolicy,
        }

        impl MakeRouteLayer for $make {
            type Layer = $layer<HeaderValue>;

            fn make(&self, args: Args) -> Result<Self::Layer, Error> {
                let config = Config::with_args(args).map_err(Error::new)?;
                let name = HeaderName::from_str(&config.name).map_err(Error::new)?;
                let value = HeaderValue::from_str(&config.value).map_err(Error::new)?;
                Ok(match config.policy {
                    SetPolicy::Overriding => $layer::overriding(name, value),
                    SetPolicy::Appending => $layer::appending(name, value),
                    SetPolicy::IfNotPresent => $layer::if_not_present(name, value),
                })
            }
        }
    };
}

mod request {
    use crate::make::MakeRouteLayer;
    use crate::util::SetPolicy;
    use http::{HeaderName, HeaderValue};
    use satex_core::component::{Args, Configurable};
    use satex_core::Error;
    use satex_macro::make;
    use std::str::FromStr;
    use tower_http::set_header::SetRequestHeaderLayer;

    make_set_header!(
        SetRequestHeader,
        MakeSetRequestHeaderRouteLayer,
        SetRequestHeaderLayer
    );
}

mod response {
    use crate::make::MakeRouteLayer;
    use crate::util::SetPolicy;
    use http::{HeaderName, HeaderValue};
    use satex_core::component::{Args, Configurable};
    use satex_core::Error;
    use satex_macro::make;
    use std::str::FromStr;
    use tower_http::set_header::SetResponseHeaderLayer;

    make_set_header!(
        SetResponseHeader,
        MakeSetResponseHeaderRouteLayer,
        SetResponseHeaderLayer
    );
}
