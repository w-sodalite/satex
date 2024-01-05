use hyper::Request;

use regex::Regex;
use satex_core::http::Body;
use satex_core::Error;

use crate::RouteMatcher;
pub use make::MakePathMatcher;

mod make;

pub enum Pattern {
    StartsWith(String),
    Regex(Regex),
}

pub struct PathMatcher {
    patterns: Vec<Pattern>,
}

impl PathMatcher {
    pub fn new(patterns: Vec<Pattern>) -> Self {
        Self { patterns }
    }
}

impl RouteMatcher for PathMatcher {
    fn is_match(&self, request: &Request<Body>) -> Result<bool, Error> {
        let path = request.uri().path();
        for pattern in self.patterns.iter() {
            match pattern {
                Pattern::StartsWith(pattern) => {
                    if path.starts_with(pattern) {
                        return Ok(true);
                    }
                }
                Pattern::Regex(regex) => {
                    if regex.is_match(path) {
                        return Ok(true);
                    }
                }
            }
        }
        Ok(false)
    }
}
