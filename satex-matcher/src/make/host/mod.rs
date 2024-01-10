use hyper::header::HOST;
use regex::Regex;

pub use make::MakeHostMatcher;
use satex_core::essential::Essential;
use satex_core::{satex_error, Error};

use crate::RouteMatcher;

mod make;

pub struct HostMatcher {
    patterns: Vec<Regex>,
}

impl HostMatcher {
    pub fn new(patterns: Vec<Regex>) -> Self {
        Self { patterns }
    }
}

impl RouteMatcher for HostMatcher {
    fn is_match(&self, essential: &mut Essential) -> Result<bool, Error> {
        let host = essential.headers.get(HOST);
        match host {
            None => Ok(false),
            Some(host) => host
                .to_str()
                .map(|host| self.patterns.iter().any(|pattern| pattern.is_match(host)))
                .map_err(|e| satex_error!(e)),
        }
    }
}
