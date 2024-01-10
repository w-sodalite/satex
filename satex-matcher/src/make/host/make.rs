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
    use hyper::header::{HeaderValue, HOST};

    use satex_core::config::args::{Args, Shortcut};

    use crate::make::host::MakeHostMatcher;
    use crate::{MakeRouteMatcher, RouteMatcher, __assert_matcher};

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::from("127.0.0.([1-9])"));
        __assert_matcher!(
            MakeHostMatcher,
            args,
            [
                Ok(true) => |e| {
                    e.headers
                        .insert(HOST, HeaderValue::from_static("127.0.0.1"))
                },
                Ok(true) => |e| {
                    e.headers
                        .insert(HOST, HeaderValue::from_static("127.0.0.2"))
                },
                Ok(false) => |e| {
                    e.headers
                        .insert(HOST, HeaderValue::from_static("127.0.1.1"))
                }
            ]
        );
    }
}
