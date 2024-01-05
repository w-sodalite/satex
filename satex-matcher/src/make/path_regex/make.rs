use regex::Regex;

use satex_core::config::args::Args;
use satex_core::serde::regex::SerdeRegex;
use satex_core::Error;

use crate::make::path_regex::PathRegexMatcher;
use crate::{MakeRouteMatcher, __matcher};

__matcher! {
    PathRegex,
    List,
    patterns: Vec<SerdeRegex>
}

fn make(args: Args) -> Result<PathRegexMatcher, Error> {
    let config = Config::try_from(args)?;
    Ok(PathRegexMatcher::new(
        config.patterns.into_iter().map(Regex::from).collect(),
    ))
}

#[cfg(test)]
mod test {
    use hyper::Request;
    use satex_core::{
        config::args::{Args, Shortcut},
        http::Body,
    };

    use crate::{MakeRouteMatcher, RouteMatcher};

    use super::MakePathRegexMatcher;

    fn new_request(path: &str) -> Request<Body> {
        Request::builder()
            .uri(format!("http://test{}", path))
            .body(Body::empty())
            .unwrap()
    }

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::new("/a(\\/*)(.*)"));
        let make = MakePathRegexMatcher::default();
        let matcher = make.make(args).unwrap();
        assert!(matcher.is_match(&new_request("/a")).unwrap());
        assert!(!matcher.is_match(&new_request("/b")).unwrap());
        assert!(matcher.is_match(&new_request("/a/b")).unwrap());
        assert!(matcher.is_match(&new_request("/a/b/c")).unwrap());
        assert!(!matcher.is_match(&new_request("/b/c")).unwrap());
    }
}
