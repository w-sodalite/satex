use regex::Regex;
use satex_core::{apply::Apply, config::args::Args};
use satex_core::{satex_error, Error};

use crate::{MakeRouteMatcher, __matcher};

use super::{PathMatcher, Pattern};

const STARTS_WITH_PREFIX: &str = "@";

__matcher! {
    Path,
    List,
    patterns: Vec<String>
}

fn make(args: Args) -> Result<PathMatcher, Error> {
    let config = Config::try_from(args)?;
    let mut patterns = vec![];
    for pattern in config.patterns.into_iter() {
        if pattern.starts_with(STARTS_WITH_PREFIX) {
            patterns.push(Pattern::StartsWith(
                pattern.apply(|pattern| pattern.remove(0)),
            ));
        } else {
            patterns.push(Pattern::Regex(
                Regex::new(&pattern).map_err(|e| satex_error!(e))?,
            ))
        }
    }
    Ok(PathMatcher::new(patterns))
}

#[cfg(test)]
mod test {
    use hyper::Request;
    use satex_core::{
        config::args::{Args, Shortcut},
        http::Body,
    };

    use crate::{make::path::make::MakePathMatcher, MakeRouteMatcher, RouteMatcher};

    fn new_request(path: &str) -> Request<Body> {
        Request::builder()
            .uri(format!("http://test{}", path))
            .body(Body::empty())
            .unwrap()
    }

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::from("@/a,/b"));
        let make = MakePathMatcher::default();
        let matcher = make.make(args).unwrap();
        assert!(matcher.is_match(&new_request("/a")).unwrap());
        assert!(matcher.is_match(&new_request("/b")).unwrap());
        assert!(matcher.is_match(&new_request("/a/b")).unwrap());
        assert!(matcher.is_match(&new_request("/b/c")).unwrap());
        assert!(!matcher.is_match(&new_request("/c")).unwrap());
    }
}
