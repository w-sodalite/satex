use crate::make::MakeRouteMatcher;
use crate::RouteMatcher;
use async_trait::async_trait;
use http::request::Parts;
use qstring::QString;
use satex_core::component::{Args, Configurable};
use satex_core::expression::Expression;
use satex_core::Error;
use satex_macro::{Configurable, Make};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct QueryRouteMatcher {
    name: String,
    value: Expression,
}

impl QueryRouteMatcher {
    pub fn new(name: String, value: Expression) -> Self {
        Self { name, value }
    }
}

#[async_trait]
impl RouteMatcher for QueryRouteMatcher {
    async fn matches(&self, parts: &mut Parts) -> Result<bool, Error> {
        let query = match parts.uri.query() {
            Some(query) => QString::from(query),
            None => QString::default(),
        };
        Ok(self.value.matches(query.get(&self.name)))
    }
}

#[derive(Deserialize, Configurable)]
#[configurable(companion = "MakeQueryRouteMatcher")]
struct Config {
    name: String,
    value: Expression,
}

#[derive(Debug, Clone, Make)]
#[make(name = "Query")]
pub struct MakeQueryRouteMatcher;

impl MakeRouteMatcher for MakeQueryRouteMatcher {
    type Matcher = QueryRouteMatcher;

    fn make(&self, args: Args) -> Result<Self::Matcher, Error> {
        Config::with_args(args).map(|config| QueryRouteMatcher::new(config.name, config.value))
    }
}
