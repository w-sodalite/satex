#![doc = include_str!("../../docs/between.md")]

use crate::make::MakeRouteMatcher;
use crate::RouteMatcher;
use async_trait::async_trait;
use chrono::{Local, NaiveDateTime};
use http::request::Parts;
use satex_core::component::{Args, Configurable};
use satex_core::Error;
use satex_macro::make;

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

#[make(kind = Between)]
pub struct MakeBetweenRouteMatcher {
    start: NaiveDateTime,
    end: NaiveDateTime,
}

impl MakeRouteMatcher for MakeBetweenRouteMatcher {
    type Matcher = BetweenRouteMatcher;

    fn make(&self, args: Args) -> Result<Self::Matcher, Error> {
        Config::with_args(args).map(|config| BetweenRouteMatcher::new(config.start, config.end))
    }
}
