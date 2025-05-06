#![doc = include_str!("../docs/header.md")]

use crate::make::MakeRouteMatcher;
use crate::RouteMatcher;
use async_trait::async_trait;
use http::request::Parts;
use satex_core::component::{Args, Configurable};
use satex_core::expression::Expression;
use satex_core::Error;
use satex_macro::make;

pub struct HeaderRouteMatcher {
    name: String,
    value: Expression,
}

impl HeaderRouteMatcher {
    pub fn new(name: String, value: Expression) -> Self {
        Self { name, value }
    }
}

#[async_trait]
impl RouteMatcher for HeaderRouteMatcher {
    async fn matches(&self, parts: &mut Parts) -> Result<bool, Error> {
        let value = match parts.headers.get(&self.name) {
            Some(value) => Some(value.to_str().map_err(Error::new)?),
            None => None,
        };
        Ok(self.value.matches(value))
    }
}

#[make(kind = Header)]
struct MakeHeaderRouteMatcher {
    name: String,
    value: Expression,
}

impl MakeRouteMatcher for MakeHeaderRouteMatcher {
    type Matcher = HeaderRouteMatcher;

    fn make(&self, args: Args) -> Result<Self::Matcher, Error> {
        Config::with_args(args).map(|config| HeaderRouteMatcher::new(config.name, config.value))
    }
}
