use std::collections::HashSet;

use hyper::Request;

pub use make::MakePathStartsWithMatcher;
use satex_core::http::Body;
use satex_core::Error;

use crate::RouteMatcher;

mod make;

pub struct PathStartsWithMatcher {
    paths: HashSet<String>,
}

impl PathStartsWithMatcher {
    pub fn new(paths: HashSet<String>) -> Self {
        Self { paths }
    }
}

impl RouteMatcher for PathStartsWithMatcher {
    fn is_match(&self, request: &Request<Body>) -> Result<bool, Error> {
        let path = request.uri().path();
        Ok(self
            .paths
            .iter()
            .any(|item| path.starts_with(item.as_str())))
    }
}
