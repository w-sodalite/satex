use cookie::Cookie;
use hyper::header::COOKIE;

use satex_core::essential::Essential;
use satex_core::pattern::Patterns;
use satex_core::{satex_error, Error};

use crate::RouteMatcher;

mod make;

pub struct CookieMatcher {
    name: String,
    patterns: Patterns,
}

impl CookieMatcher {
    pub fn new(name: String, patterns: Patterns) -> Self {
        Self { name, patterns }
    }
}

impl RouteMatcher for CookieMatcher {
    fn is_match(&self, essential: &mut Essential) -> Result<bool, Error> {
        let value = match essential.headers.get(COOKIE) {
            Some(cookie) => {
                let cookie = cookie.to_str().map_err(|e| satex_error!(e))?;
                Cookie::split_parse(cookie).try_fold(None, |mut target, cookie| match cookie {
                    Ok(cookie) => {
                        if cookie.name() == self.name {
                            Ok(Some(cookie.value().to_string()))
                        } else {
                            Ok(None)
                        }
                    }
                    Err(e) => Err(satex_error!(e)),
                })?
            }
            None => None,
        };
        Ok(self
            .patterns
            .iter()
            .any(|pattern| pattern.is_match(value.as_ref())))
    }
}
