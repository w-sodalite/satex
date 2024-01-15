use hyper::header::HOST;

pub use make::MakeHostMatcher;
use satex_core::essential::Essential;
use satex_core::pattern::Pattern;
use satex_core::{satex_error, Error};

use crate::RouteMatcher;

mod make;

pub struct HostMatcher {
    values: Vec<Pattern>,
}

impl HostMatcher {
    pub fn new(values: Vec<Pattern>) -> Self {
        Self { values }
    }
}

impl RouteMatcher for HostMatcher {
    fn is_match(&self, essential: &mut Essential) -> Result<bool, Error> {
        let value = match essential.headers.get(HOST) {
            Some(host) => Some(host.to_str().map_err(|e| satex_error!(e))?),
            None => None,
        };
        Ok(self.values.iter().any(|pattern| pattern.is_match(value)))
    }
}
