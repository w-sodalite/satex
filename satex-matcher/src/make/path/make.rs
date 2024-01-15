use satex_core::config::args::Args;
use satex_core::Error;

use crate::{MakeRouteMatcher, __make_matcher};

use super::PathMatcher;

__make_matcher! {
    Path,
    Sequence,
    patterns: Vec<String>
}

fn make(args: Args) -> Result<PathMatcher, Error> {
    let config = Config::try_from(args)?;
    Ok(PathMatcher::new(config.patterns))
}

#[cfg(test)]
mod test {
    use hyper::Uri;

    use satex_core::config::args::{Args, Shortcut};

    use crate::{MakeRouteMatcher, RouteMatcher, __assert_matcher};

    use super::MakePathMatcher;

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::from("/a,/b/:x,/c/:x+"));
        __assert_matcher!(
            MakePathMatcher,
            args,
            [
                Ok(true) => |e| { e.uri = Uri::from_static("https://satex.dev/a") },
                Ok(false) => |e| { e.uri = Uri::from_static("https://satex.dev/a/b") },
                Ok(true) => |e| { e.uri = Uri::from_static("https://satex.dev/b/c") },
                Ok(false) => |e| { e.uri = Uri::from_static("https://satex.dev/b/c/d") },
                Ok(true) => |e| { e.uri = Uri::from_static("https://satex.dev/c/d/e/f/g") },
            ]
        );
    }
}
