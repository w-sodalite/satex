use crate::make::MakeRouteLayer;
use crate::util::SetPolicy;
use http::{HeaderName, HeaderValue};
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::{Configurable, Make};
use serde::Deserialize;
use std::str::FromStr;
use tower_http::set_header::{SetRequestHeaderLayer, SetResponseHeaderLayer};

#[derive(Deserialize, Configurable)]
#[configurable(companion = "SetRequestHeader", shortcut_mode = "Object")]
struct Config {
    name: String,
    value: String,
    #[serde(default)]
    policy: SetPolicy,
}

macro_rules! make_set_header {
    ($name:ident,$ident:ident,$layer:ident) => {
        #[derive(Debug, Clone, Copy, Default, Make)]
        #[make(name = stringify!($name))]
        pub struct $ident;

        impl MakeRouteLayer for $ident {
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

make_set_header!(
    SetRequestHeader,
    MakeSetRequestHeaderRouteLayer,
    SetRequestHeaderLayer
);

make_set_header!(
    SetResponseHeader,
    MakeSetResponseHeaderRouteLayer,
    SetResponseHeaderLayer
);
