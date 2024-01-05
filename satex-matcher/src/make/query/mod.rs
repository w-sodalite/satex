use hyper::Request;

pub use make::MakeQueryMatcher;
use regex::Regex;
use satex_core::http::Body;
use satex_core::Error;

use crate::RouteMatcher;

mod make;

#[derive(Clone)]
pub struct QueryMatcher {
    name: String,
    value: Regex,
}

impl QueryMatcher {
    pub fn new(name: String, value: Regex) -> Self {
        Self { name, value }
    }
}

impl RouteMatcher for QueryMatcher {
    fn is_match(&self, request: &Request<Body>) -> Result<bool, Error> {
        match request.uri().query() {
            Some(query) => {
                let query = qstring::QString::from(query);
                let matched = query
                    .get(&self.name)
                    .map_or_else(|| false, |value| self.value.is_match(value));
                Ok(matched)
            }
            None => Ok(false),
        }
    }
}
