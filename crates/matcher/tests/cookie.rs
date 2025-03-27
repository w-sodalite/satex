mod util;

use crate::util::parts;
use http::header::COOKIE;
use http::request::Parts;
use http::{HeaderValue, Method};
use satex_core::component::Args;
use satex_matcher::cookie::MakeCookieRouteMatcher;
use satex_matcher::make::MakeRouteMatcher;
use satex_matcher::RouteMatcher;
use serde_yaml::Value;

async fn matches_shortcut(parts: &mut Parts, name: &str, value: &str) {
    let shortcut = format!("{}, {}", name, value);
    let args = Args::shortcut(&shortcut);
    let matcher = MakeCookieRouteMatcher.make(args).unwrap();
    let flag = matcher.matches(parts).await.unwrap();
    assert!(flag);
}

async fn make_full(parts: &mut Parts, name: &str, value: &str) {
    let yaml = format!(
        r#"
            name: {}
            value: {}
          "#,
        name, value
    );
    let value = serde_yaml::from_str::<Value>(&yaml).unwrap();
    let args = Args::full(&value);
    let matcher = MakeCookieRouteMatcher.make(args).unwrap();
    let flag = matcher.matches(parts).await.unwrap();
    assert!(flag);
}

#[tokio::test]
async fn make_with_shortcut() {
    let mut parts = parts("/?name=admin&age=12", Method::GET);
    parts
        .headers
        .insert(COOKIE, HeaderValue::from_static("x-data=hello world"));

    matches_shortcut(&mut parts, "x-data", "Equals(hello world)").await;
    matches_shortcut(&mut parts, "x-data", "NotEquals(HELLO WORLD)").await;
    matches_shortcut(&mut parts, "x-data", "StartsWith(hello)").await;
    matches_shortcut(&mut parts, "x-data", "NotStartsWith(world)").await;
    matches_shortcut(&mut parts, "x-data", "EndsWith(world)").await;
    matches_shortcut(&mut parts, "x-data", "NotEndsWith(hello)").await;
    matches_shortcut(&mut parts, "x-data", "Contains(hello)").await;
    matches_shortcut(&mut parts, "x-data", "NotContains(Hello)").await;
    matches_shortcut(&mut parts, "x-data", "Exists").await;

    parts.headers.remove(COOKIE);
    matches_shortcut(&mut parts, "x-data", "NotExists").await;
}

#[tokio::test]
async fn make_with_full() {
    let mut parts = parts("/?name=admin&age=12", Method::GET);
    parts
        .headers
        .insert(COOKIE, HeaderValue::from_static("x-data=hello world"));

    make_full(&mut parts, "x-data", "Equals(hello world)").await;
    make_full(&mut parts, "x-data", "NotEquals(HELLO WORLD)").await;
    make_full(&mut parts, "x-data", "StartsWith(hello)").await;
    make_full(&mut parts, "x-data", "NotStartsWith(world)").await;
    make_full(&mut parts, "x-data", "EndsWith(world)").await;
    make_full(&mut parts, "x-data", "NotEndsWith(hello)").await;
    make_full(&mut parts, "x-data", "Contains(hello)").await;
    make_full(&mut parts, "x-data", "NotContains(Hello)").await;
    make_full(&mut parts, "x-data", "Exists").await;

    parts.headers.remove(COOKIE);
    matches_shortcut(&mut parts, "x-data", "NotExists").await;
}
