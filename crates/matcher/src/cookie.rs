use crate::make::MakeRouteMatcher;
use crate::RouteMatcher;
use async_trait::async_trait;
use cookie::Cookie;
use http::header::COOKIE;
use http::request::Parts;
use satex_core::component::{Args, Configurable};
use satex_core::expression::Expression;
use satex_core::Error;
use satex_macro::make;

pub struct CookieRouteMatcher {
    name: String,
    value: Expression,
}

impl CookieRouteMatcher {
    pub fn new(name: String, value: Expression) -> Self {
        Self { name, value }
    }
}

#[async_trait]
impl RouteMatcher for CookieRouteMatcher {
    async fn matches(&self, parts: &mut Parts) -> Result<bool, Error> {
        let value = match parts.headers.get(COOKIE) {
            Some(value) => {
                let cookies = Cookie::split_parse(value.to_str().map_err(Error::new)?);
                cookies.into_iter().find_map(|cookie| match cookie {
                    Ok(cookie) => {
                        if cookie.name() == self.name {
                            Some(cookie.value().to_string())
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                })
            }
            None => None,
        };
        Ok(self.value.matches(value.as_deref()))
    }
}

#[make(kind = Cookie)]
struct MakeCookieRouteMatcher {
    name: String,
    value: Expression,
}

impl MakeRouteMatcher for MakeCookieRouteMatcher {
    type Matcher = CookieRouteMatcher;

    fn make(&self, args: Args) -> Result<Self::Matcher, Error> {
        Config::with_args(args).map(|config| CookieRouteMatcher::new(config.name, config.value))
    }
}
