use hyper::Request;
use regex::Regex;

pub use make::MakePathRegexMatcher;
use satex_core::http::Body;
use satex_core::Error;

use crate::RouteMatcher;

mod make;

pub struct PathRegexMatcher {
    patterns: Vec<Regex>,
}

impl PathRegexMatcher {
    pub fn new(patterns: Vec<Regex>) -> Self {
        Self { patterns }
    }
}

impl RouteMatcher for PathRegexMatcher {
    fn is_match(&self, request: &Request<Body>) -> Result<bool, Error> {
        Ok(self
            .patterns
            .iter()
            .any(|regex| regex.is_match(request.uri().path())))
    }
}
