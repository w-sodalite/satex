mod util;

use crate::util::parts;
use http::request::Parts;
use http::Method;
use satex_core::component::Args;
use satex_matcher::make::MakeRouteMatcher;
use satex_matcher::query::MakeQueryRouteMatcher;
use satex_matcher::RouteMatcher;
use serde_yaml::Value;

async fn matches_shortcut(parts: &mut Parts, name: &str, value: &str) {
    let shortcut = format!("{}, {}", name, value);
    let args = Args::shortcut(&shortcut);
    let matcher = MakeQueryRouteMatcher.make(args).unwrap();
    let flag = matcher.matches(parts).await.unwrap();
    assert!(flag);
}

async fn matches_full(parts: &mut Parts, name: &str, value: &str) {
    let yaml = format!(
        r#"
            name: {}
            value: {}
        "#,
        name, value
    );
    let value = serde_yaml::from_str::<Value>(&yaml).unwrap();
    let args = Args::full(&value);
    let matcher = MakeQueryRouteMatcher.make(args).unwrap();
    let flag = matcher.matches(parts).await.unwrap();
    assert!(flag);
}

#[tokio::test]
async fn make_with_shortcut() {
    let mut parts = parts("/?name=admin&age=12", Method::GET);

    matches_shortcut(&mut parts, "name", "Equals(admin)").await;
    matches_shortcut(&mut parts, "name", "?Contains(Adm)").await;
    matches_shortcut(&mut parts, "name", "NotContains(Adm)").await;
    matches_shortcut(&mut parts, "name", "?StartsWith(Adm)").await;
    matches_shortcut(&mut parts, "name", "NotStartsWith(Adm)").await;
    matches_shortcut(&mut parts, "name", "?EndsWith(Min)").await;
    matches_shortcut(&mut parts, "name", "NotEndsWith(Min)").await;
    matches_shortcut(&mut parts, "name", "Regex(^admin$)").await;
    matches_shortcut(&mut parts, "name", "Exists").await;
    matches_shortcut(&mut parts, "password", "NotExists").await;
}

#[tokio::test]
async fn make_with_full() {
    let mut parts = parts("/?name=admin&age=12", Method::GET);

    matches_full(&mut parts, "name", "Equals(admin)").await;
    matches_full(&mut parts, "name", "?Contains(Adm)").await;
    matches_full(&mut parts, "name", "NotContains(Adm)").await;
    matches_full(&mut parts, "name", "?StartsWith(Adm)").await;
    matches_full(&mut parts, "name", "NotStartsWith(Adm)").await;
    matches_full(&mut parts, "name", "?EndsWith(Min)").await;
    matches_full(&mut parts, "name", "NotEndsWith(Min)").await;
    matches_full(&mut parts, "name", "Regex(^admin$)").await;
    matches_full(&mut parts, "name", "Exists").await;
    matches_full(&mut parts, "password", "NotExists").await;
}
