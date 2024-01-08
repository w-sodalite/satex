use satex_core::config::args::Args;
use satex_core::serde::regex::SerdeRegex;
use satex_core::Error;

use crate::{MakeRouteMatcher, __matcher};

use super::CookieMatcher;

__matcher! {
    Cookie,
    name: String,
    value: SerdeRegex
}

fn make(args: Args<'_>) -> Result<CookieMatcher, Error> {
    let config = Config::try_from(args)?;
    Ok(CookieMatcher::new(config.name, config.value.into()))
}

#[cfg(test)]
mod test {
    use hyper::Request;

    use satex_core::config::args::{Args, Shortcut};
    use satex_core::http::Body;

    use crate::make::cookie::make::MakeCookieMatcher;
    use crate::{MakeRouteMatcher, RouteMatcher};

    fn new_request(key: &str, value: &str) -> Request<Body> {
        Request::builder()
            .header("cookie", format!("{}={}", key, value))
            .body(Body::empty())
            .unwrap()
    }

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::from("k1,v1"));
        let make = MakeCookieMatcher::default();
        let matcher = make.make(Args::from(args)).unwrap();
        assert!(matcher.is_match(&new_request("k1", "v1")).unwrap());
        assert!(!matcher.is_match(&new_request("k2", "v2")).unwrap());
    }
}
