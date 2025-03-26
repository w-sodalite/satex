use crate::make::MakeRouteMatcher;
use crate::RouteMatcher;
use async_trait::async_trait;
use chrono::{Local, NaiveDateTime};
use http::request::Parts;
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::{Configurable, Make};
use serde::Deserialize;

pub struct BetweenRouteMatcher {
    start: NaiveDateTime,
    end: NaiveDateTime,
}

impl BetweenRouteMatcher {
    pub fn new(start: NaiveDateTime, end: NaiveDateTime) -> Self {
        Self { start, end }
    }
}

#[async_trait]
impl RouteMatcher for BetweenRouteMatcher {
    async fn matches(&self, _: &mut Parts) -> Result<bool, Error> {
        let now = Local::now().naive_local();
        Ok(now >= self.start && now <= self.end)
    }
}

#[derive(Deserialize, Configurable)]
#[configurable(companion = "")]
struct Config {
    start: NaiveDateTime,
    end: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, Default, Make)]
#[make(name = "Between")]
pub struct MakeBetweenRouteMatcher;

impl MakeRouteMatcher for MakeBetweenRouteMatcher {
    type Matcher = BetweenRouteMatcher;

    fn make(&self, args: Args) -> Result<Self::Matcher, Error> {
        Config::with_args(args).map(|config| BetweenRouteMatcher::new(config.start, config.end))
    }
}
