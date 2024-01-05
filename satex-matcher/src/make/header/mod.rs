use hyper::http::HeaderName;
use hyper::Request;
use regex::Regex;

pub use make::MakeHeaderMatcher;
use satex_core::http::Body;
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
    fn is_match(&self, request: &Request<Body>) -> Result<bool, Error> {
        match request.headers().get(&self.name) {
            Some(value) => value
                .to_str()
                .map_err(|e| satex_error!(e))
                .map(|value| self.value.is_match(value)),
            None => Ok(false),
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use hyper::http::HeaderName;
    use hyper::Request;
    use regex::Regex;

    use satex_core::http::Body;

    use crate::make::header::HeaderMatcher;
    use crate::RouteMatcher;

    #[test]
    fn is_match() {
        let matcher = HeaderMatcher {
            name: HeaderName::from_static("k1"),
            value: Regex::from_str("v1").unwrap(),
        };
        let request = Request::builder()
            .header("k1", "v1")
            .body(Body::empty())
            .unwrap();
        assert!(matcher.is_match(&request).unwrap());
    }
}
