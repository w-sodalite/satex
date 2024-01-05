use satex_core::config::args::Args;
use satex_core::serde::http::SerdeHeaderName;
use satex_core::serde::regex::SerdeRegex;
use satex_core::Error;

use crate::make::header::HeaderMatcher;
use crate::{MakeRouteMatcher, __matcher};

__matcher! {
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

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::new("k1,v1"));
        let matcher = MakeHeaderMatcher.make(Args::from(args)).unwrap();
        let request = Request::builder()
            .header("k1", "v1")
            .body(Body::empty())
            .unwrap();
        assert!(matcher.is_match(&request).unwrap());
    }
}
