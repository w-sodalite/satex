use chrono::NaiveDateTime;

use satex_core::config::args::Args;
use satex_core::satex_error;
use satex_core::Error;

use crate::make::time::{Mode, TimeMatcher};
use crate::{MakeRouteMatcher, __matcher};

const DEFAULT_TIME_PATTERN: &str = "%Y-%m-%d %H:%M:%S";

__matcher! {
    Time,
    mode: Mode,
    time: String,
}

fn make(args: Args) -> Result<TimeMatcher, Error> {
    let config = Config::try_from(args)?;
    let time = NaiveDateTime::parse_from_str(&config.time, DEFAULT_TIME_PATTERN)
        .map_err(|e| satex_error!(e))?;
    Ok(TimeMatcher::new(config.mode, time))
}

#[cfg(test)]
mod test {
    use std::{
        ops::{Add, Sub},
        time::Duration,
    };

    use chrono::Local;
    use hyper::Request;
    use satex_core::{
        config::args::{Args, Shortcut},
        http::Body,
    };

    use crate::{MakeRouteMatcher, RouteMatcher};

    use super::{MakeTimeMatcher, DEFAULT_TIME_PATTERN};

    fn new_request() -> Request<Body> {
        Request::builder().body(Body::empty()).unwrap()
    }

    #[test]
    fn test_match() {
        let make = MakeTimeMatcher::default();
        let now = Local::now();
        let m5 = Duration::from_secs(300);
        let before = now.sub(m5).format(DEFAULT_TIME_PATTERN).to_string();
        let after: String = now.add(m5).format(DEFAULT_TIME_PATTERN).to_string();
        let before = format!("After,{}", before);
        let args = Args::Shortcut(Shortcut::new(&before));
        let matcher = make.make(args).unwrap();
        assert!(matcher.is_match(&new_request()).unwrap());

        let after = format!("Before,{}", after);
        let args = Args::Shortcut(Shortcut::new(&after));
        let matcher = make.make(args).unwrap();
        assert!(matcher.is_match(&new_request()).unwrap());
    }
}