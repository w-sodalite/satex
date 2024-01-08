use satex_core::config::args::Args;
use satex_core::serde::regex::SerdeRegex;
use satex_core::Error;

use crate::make::query::QueryMatcher;
use crate::{MakeRouteMatcher, __matcher};

__matcher! {
    Query,
    name: String,
    value: SerdeRegex,
}

fn make(args: Args) -> Result<QueryMatcher, Error> {
    let config = Config::try_from(args)?;
    Ok(QueryMatcher::new(config.name, config.value.into()))
}

#[cfg(test)]
mod test {
    use hyper::Request;
    use satex_core::{
        config::args::{Args, Shortcut},
        http::Body,
    };

    use crate::{MakeRouteMatcher, RouteMatcher};

    use super::MakeQueryMatcher;

    fn new_request(key: &str, value: &str) -> Request<Body> {
        Request::builder()
            .uri(format!("https://www.rust-lang.org?{}={}", key, value))
            .body(Body::empty())
            .unwrap()
    }

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::from("k1,v1"));
        let make = MakeQueryMatcher::default();
        let matcher = make.make(args).unwrap();
        assert!(matcher.is_match(&new_request("k1", "v1")).unwrap());
        assert!(!matcher.is_match(&new_request("k1", "v2")).unwrap());
        assert!(!matcher.is_match(&new_request("k2", "v2")).unwrap());
    }
}
