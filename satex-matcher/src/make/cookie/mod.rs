use cookie::Cookie;
use hyper::header::COOKIE;
use regex::Regex;
use satex_core::satex_error;

use crate::RouteMatcher;

mod make;

pub struct CookieMatcher {
    name: String,
    value: Regex,
}

impl CookieMatcher {
    pub fn new(name: String, value: Regex) -> Self {
        Self { name, value }
    }
}

impl RouteMatcher for CookieMatcher {
    fn is_match(
        &self,
        request: &hyper::Request<satex_core::http::Body>,
    ) -> Result<bool, satex_core::Error> {
        match request.headers().get(COOKIE) {
            Some(value) => {
                let value = value.to_str().map_err(|e| satex_error!(e))?;
                Ok(Cookie::split_parse(value)
                    .flatten()
                    .filter(|cookie| cookie.name() == self.name.as_str())
                    .any(|cookie| self.value.is_match(cookie.value())))
            }
            None => Ok(false),
        }
    }
}
