use hyper::http::HeaderName;
use regex::Regex;

pub use make::MakeHeaderMatcher;
use satex_core::essential::Essential;
use satex_core::{satex_error, Error};

use crate::RouteMatcher;

mod make;

pub struct HeaderMatcher {
    name: HeaderName,
    value: Regex,
}

impl HeaderMatcher {
    pub fn new(name: HeaderName, value: Regex) -> Self {
        Self { name, value }
    }
}

impl RouteMatcher for HeaderMatcher {
    fn is_match(&self, essential: &mut Essential) -> Result<bool, Error> {
        match essential.headers.get(&self.name) {
            Some(value) => value
                .to_str()
                .map_err(|e| satex_error!(e))
                .map(|value| self.value.is_match(value)),
            None => Ok(false),
        }
    }
}
