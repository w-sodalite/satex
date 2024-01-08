use regex::Regex;

use satex_core::config::args::Args;
use satex_core::serde::regex::SerdeRegex;
use satex_core::Error;

use crate::make::host::HostMatcher;
use crate::{MakeRouteMatcher, __make_matcher};

__make_matcher! {
    Host,
    List,
    patterns: Vec<SerdeRegex>
}

fn make(args: Args<'_>) -> Result<HostMatcher, Error> {
    let config = Config::try_from(args)?;
    Ok(HostMatcher::new(
        config.patterns.into_iter().map(Regex::from).collect(),
    ))
}

#[cfg(test)]
mod test {
    use hyper::{header::HOST, Request};
    use satex_core::{
        config::args::{Args, Shortcut},
        http::Body,
    };

    use crate::{MakeRouteMatcher, RouteMatcher};

    use super::MakeHostMatcher;

    fn new_request(host: &str) -> Request<Body> {
        Request::builder()
            .header(HOST, host)
            .body(Body::empty())
            .unwrap()
    }

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::from("127.0.0.([1-9])"));
        let make = MakeHostMatcher::default();
        let matcher = make.make(args).unwrap();
        assert!(matcher.is_match(&new_request("127.0.0.1")).unwrap());
        assert!(matcher.is_match(&new_request("127.0.0.2")).unwrap());
        assert!(!matcher.is_match(&new_request("127.0.1.1")).unwrap());
    }
}
