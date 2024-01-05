use std::collections::HashSet;

use satex_core::config::args::Args;
use satex_core::Error;

use crate::make::path_starts_with::PathStartsWithMatcher;
use crate::{MakeRouteMatcher, __matcher};

__matcher! {
    PathStartsWith,
    List,
    patterns: HashSet<String>
}

fn make(args: Args) -> Result<PathStartsWithMatcher, Error> {
    let config = Config::try_from(args)?;
    Ok(PathStartsWithMatcher::new(config.patterns))
}

#[cfg(test)]
mod test {
    use hyper::Request;
    use satex_core::{
        config::args::{Args, Shortcut},
        http::Body,
    };

    use crate::{MakeRouteMatcher, RouteMatcher};

    use super::MakePathStartsWithMatcher;

    fn new_request(path: &str) -> Request<Body> {
        Request::builder()
            .uri(format!("http://test{}", path))
            .body(Body::empty())
            .unwrap()
    }

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::new("/a,/b"));
        let make = MakePathStartsWithMatcher::default();
        let matcher = make.make(args).unwrap();
        assert!(matcher.is_match(&new_request("/a")).unwrap());
        assert!(matcher.is_match(&new_request("/b")).unwrap());
        assert!(matcher.is_match(&new_request("/a/b")).unwrap());
        assert!(matcher.is_match(&new_request("/b/c")).unwrap());
        assert!(!matcher.is_match(&new_request("/c")).unwrap());
    }
}
