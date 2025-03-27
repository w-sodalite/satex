mod util;

use crate::util::parts;
use http::header::HOST;
use http::request::Parts;
use http::{HeaderValue, Method};
use satex_core::component::Args;
use satex_matcher::host::MakeHostRouteMatcher;
use satex_matcher::make::MakeRouteMatcher;
use satex_matcher::RouteMatcher;
use serde_yaml::Value;

async fn matches_shortcut(parts: &mut Parts, shortcut: &str) {
    let args = Args::shortcut(shortcut);
    let matcher = MakeHostRouteMatcher.make(args).unwrap();
    let flag = matcher.matches(parts).await.unwrap();
    assert!(flag);
}

async fn matches_full(parts: &mut Parts, value: &str) {
    let yaml = format!("value: {}", value);
    let value = serde_yaml::from_str::<Value>(&yaml).unwrap();
    let args = Args::full(&value);
    let matcher = MakeHostRouteMatcher.make(args).unwrap();
    let flag = matcher.matches(parts).await.unwrap();
    assert!(flag);
}

#[tokio::test]
async fn make_with_shortcut() {
    let mut parts = parts("/", Method::GET);
    parts
        .headers
        .insert(HOST, HeaderValue::from_static("127.0.0.1"));

    matches_shortcut(&mut parts, "Equals(127.0.0.1)").await;
    matches_shortcut(&mut parts, "NotEquals(127.0.0.2)").await;
    matches_shortcut(&mut parts, "StartsWith(127.0.0)").await;
    matches_shortcut(&mut parts, "NotStartsWith(128.0)").await;
    matches_shortcut(&mut parts, "EndsWith(0.1)").await;
    matches_shortcut(&mut parts, "NotEndsWith(1.1)").await;
    matches_shortcut(&mut parts, "Contains(0.0.1)").await;
    matches_shortcut(&mut parts, "NotContains(1.0.1)").await;
    matches_shortcut(&mut parts, "Exists").await;
    matches_shortcut(&mut parts, "Regex(^127.0.0.1$)").await;

    let _ = parts.headers.remove(HOST);
    matches_shortcut(&mut parts, "NotExists").await;
}

#[tokio::test]
async fn make_with_full() {
    let mut parts = parts("/", Method::GET);
    parts
        .headers
        .insert(HOST, HeaderValue::from_static("127.0.0.1"));

    matches_full(&mut parts, "Equals(127.0.0.1)").await;
    matches_full(&mut parts, "NotEquals(127.0.0.2)").await;
    matches_full(&mut parts, "StartsWith(127.0.0)").await;
    matches_full(&mut parts, "NotStartsWith(128.0)").await;
    matches_full(&mut parts, "EndsWith(0.1)").await;
    matches_full(&mut parts, "NotEndsWith(1.1)").await;
    matches_full(&mut parts, "Contains(0.0.1)").await;
    matches_full(&mut parts, "NotContains(1.0.1)").await;
    matches_full(&mut parts, "Exists").await;
    matches_full(&mut parts, "Regex(^127.0.0.1$)").await;

    let _ = parts.headers.remove(HOST);
    matches_full(&mut parts, "NotExists").await;
}
