use cookie::Cookie;
use hyper::header::COOKIE;

use satex_core::essential::Essential;
use satex_core::pattern::Pattern;
use satex_core::{satex_error, Error};

use crate::RouteMatcher;

mod make;

pub struct CookieMatcher {
    name: String,
    values: Vec<Pattern>,
}

impl CookieMatcher {
    pub fn new(name: String, values: Vec<Pattern>) -> Self {
        Self { name, values }
    }
}

impl RouteMatcher for CookieMatcher {
    fn is_match(&self, essential: &mut Essential) -> Result<bool, Error> {
        let cookie = match essential.headers.get(COOKIE) {
            Some(cookie) => {
                let cookie = cookie.to_str().map_err(|e| satex_error!(e))?;
                Cookie::split_parse(cookie)
                    .flatten()
                    .filter(|cookie| cookie.name() == self.name)
                    .next()
            }
            None => None,
        };
        Ok(self
            .values
            .iter()
            .any(|pattern| pattern.is_match(cookie.as_ref().map(|cookie| cookie.value()))))
    }
}
