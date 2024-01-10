use regex::Regex;

use satex_core::{apply::Apply, config::args::Args};
use satex_core::{satex_error, Error};

use crate::{MakeRouteMatcher, __make_matcher};

use super::{PathMatcher, Pattern};

const STARTS_WITH_PREFIX: &str = "@";

__make_matcher! {
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
    use hyper::Uri;

    use satex_core::config::args::{Args, Shortcut};

    use crate::{MakeRouteMatcher, RouteMatcher, __assert_matcher};

    use super::MakePathMatcher;

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::from("@/a,/b"));
        __assert_matcher!(
            MakePathMatcher,
            args,
            [
                Ok(true) => |e| { e.uri = Uri::from_static("https://satex.dev/a") },
                Ok(true) => |e| { e.uri = Uri::from_static("https://satex.dev/a/b") },
                Ok(false) => |e| { e.uri = Uri::from_static("https://satex.dev/c") },
            ]
        );
    }
}
