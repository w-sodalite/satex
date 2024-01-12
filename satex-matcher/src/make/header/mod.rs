use hyper::http::HeaderName;

pub use make::MakeHeaderMatcher;
use satex_core::essential::Essential;
use satex_core::pattern::Pattern;
use satex_core::{satex_error, Error};

use crate::RouteMatcher;

mod make;

pub struct HeaderMatcher {
    name: HeaderName,
    patterns: Vec<Pattern>,
}

impl HeaderMatcher {
    pub fn new(name: HeaderName, patterns: Vec<Pattern>) -> Self {
        Self { name, patterns }
    }
}

impl RouteMatcher for HeaderMatcher {
    fn is_match(&self, essential: &mut Essential) -> Result<bool, Error> {
        let value = match essential.headers.get(&self.name) {
            Some(value) => Some(value.to_str().map_err(|e| satex_error!(e))?),
            None => None,
        };
        Ok(self.patterns.iter().any(|pattern| pattern.is_match(value)))
    }
}
