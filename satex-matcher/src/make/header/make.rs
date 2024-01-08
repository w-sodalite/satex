use satex_core::config::args::Args;
use satex_core::serde::http::SerdeHeaderName;
use satex_core::serde::regex::SerdeRegex;
use satex_core::Error;

use crate::make::header::HeaderMatcher;
use crate::{MakeRouteMatcher, __make_matcher};

__make_matcher! {
    Header,
    name: SerdeHeaderName,
    value: SerdeRegex
}

fn make(args: Args<'_>) -> Result<HeaderMatcher, Error> {
    let config = Config::try_from(args)?;
    Ok(HeaderMatcher::new(config.name.into(), config.value.into()))
}

#[cfg(test)]
mod test {
    use hyper::Request;

    use satex_core::config::args::{Args, Shortcut};
    use satex_core::http::Body;

    use crate::make::header::MakeHeaderMatcher;
    use crate::{MakeRouteMatcher, RouteMatcher};

    fn new_request(key: &str, value: &str) -> Request<Body> {
        Request::builder()
            .header(key, value)
            .body(Body::empty())
            .unwrap()
    }

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::from("k1,v1"));
        let make = MakeHeaderMatcher::default();
        let matcher = make.make(Args::from(args)).unwrap();
        assert!(matcher.is_match(&new_request("k1", "v1")).unwrap());
        assert!(!matcher.is_match(&new_request("k2", "v2")).unwrap());
    }
}
