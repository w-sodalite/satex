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
    use hyper::http::HeaderValue;

    use satex_core::config::args::{Args, Shortcut};

    use crate::make::header::MakeHeaderMatcher;
    use crate::{MakeRouteMatcher, RouteMatcher, __assert_matcher};

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::from("k1,v1"));
        __assert_matcher!(
            MakeHeaderMatcher,
            args,
            [
                Ok(true) => |e| { e.headers.insert("k1", HeaderValue::from_static("v1")) },
                Ok(false) => |e| { e.headers.insert("k1", HeaderValue::from_static("v2")) }
            ]
        );
    }
}
