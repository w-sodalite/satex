use crate::make::MakeRouteMatcher;
use crate::RouteMatcher;
use async_trait::async_trait;
use http::request::Parts;
use qstring::QString;
use satex_core::component::{Args, Configurable};
use satex_core::expression::Expression;
use satex_core::Error;
use satex_macro::make;

#[derive(Debug, Clone)]
pub struct QueryRouteMatcher {
    name: String,
    value: Expression,
}

impl QueryRouteMatcher {
    pub fn new(name: impl Into<String>, value: Expression) -> Self {
        Self {
            name: name.into(),
            value,
        }
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

#[make(kind = Query)]
pub struct MakeQueryRouteMatcher {
    name: String,
    value: Expression,
}

impl MakeRouteMatcher for MakeQueryRouteMatcher {
    type Matcher = QueryRouteMatcher;

    fn make(&self, args: Args) -> Result<Self::Matcher, Error> {
        Config::with_args(args).map(|config| QueryRouteMatcher::new(config.name, config.value))
    }
}
