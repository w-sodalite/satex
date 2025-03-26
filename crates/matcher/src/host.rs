use crate::make::MakeRouteMatcher;
use crate::RouteMatcher;
use async_trait::async_trait;
use http::header::HOST;
use http::request::Parts;
use satex_core::component::{Args, Configurable};
use satex_core::expression::Expression;
use satex_core::Error;
use satex_macro::{Configurable, Make};
use serde::Deserialize;

pub struct HostRouteMatcher {
    value: Expression,
}

impl HostRouteMatcher {
    pub fn new(value: Expression) -> Self {
        Self { value }
    }
}

#[async_trait]
impl RouteMatcher for HostRouteMatcher {
    async fn matches(&self, parts: &mut Parts) -> Result<bool, Error> {
        let host = match parts.headers.get(HOST) {
            Some(host) => Some(host.to_str().map_err(Error::new)?),
            None => None,
        };
        Ok(self.value.matches(host))
    }
}

#[derive(Deserialize, Configurable)]
#[configurable(companion = "MakeHostRouteMatcher")]
struct Config {
    value: Expression,
}

#[derive(Debug, Clone, Copy, Default, Make)]
#[make(name = "Host")]
pub struct MakeHostRouteMatcher;

impl MakeRouteMatcher for MakeHostRouteMatcher {
    type Matcher = HostRouteMatcher;

    fn make(&self, args: Args) -> Result<Self::Matcher, Error> {
        Config::with_args(args).map(|config| HostRouteMatcher::new(config.value))
    }
}
