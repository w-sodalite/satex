use hyper::header::HOST;

pub use make::MakeHostMatcher;
use satex_core::essential::Essential;
use satex_core::pattern::Patterns;
use satex_core::{satex_error, Error};

use crate::RouteMatcher;

mod make;

pub struct HostMatcher {
    patterns: Patterns,
}

impl HostMatcher {
    pub fn new(patterns: Patterns) -> Self {
        Self { patterns }
    }
}

impl RouteMatcher for HostMatcher {
    fn is_match(&self, essential: &mut Essential) -> Result<bool, Error> {
        let value = match essential.headers.get(HOST) {
            Some(host) => {
                let value = host.to_str().map_err(|e| satex_error!(e))?;
            }
            None => None,
        };
        Ok(self.patterns.iter().any(|pattern| pattern.is_match(value)))
    }
}
