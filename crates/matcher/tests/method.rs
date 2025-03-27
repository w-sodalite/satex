use crate::util::parts;
use http::Method;
use satex_core::component::Args;
use satex_matcher::make::MakeRouteMatcher;
use satex_matcher::method::{MakeMethodRouteMatcher, MethodRouteMatcher};
use satex_matcher::RouteMatcher;
use serde_yaml::Value;

mod util;

async fn matches(matcher: &MethodRouteMatcher, method: Method, matched: bool) {
    let flag = matcher.matches(&mut parts("/", method)).await.unwrap();
    match matched {
        true => assert!(flag),
        false => assert!(!flag),
    }
}

#[tokio::test]
async fn make_with_shortcut() {
    let args = Args::shortcut("GET,POST");
    let matcher = MakeMethodRouteMatcher.make(args).unwrap();

    matches(&matcher, Method::GET, true).await;
    matches(&matcher, Method::POST, true).await;
    matches(&matcher, Method::PUT, false).await;
    matches(&matcher, Method::DELETE, false).await;
    matches(&matcher, Method::PATCH, false).await;
    matches(&matcher, Method::TRACE, false).await;
    matches(&matcher, Method::OPTIONS, false).await;
}

#[tokio::test]
async fn make_with_full() {
    let yaml = r#"
        methods:
            - GET
            - POST
    "#;
    let value = serde_yaml::from_str::<Value>(yaml).unwrap();
    let args = Args::full(&value);
    let matcher = MakeMethodRouteMatcher.make(args).unwrap();
    matches(&matcher, Method::GET, true).await;
    matches(&matcher, Method::POST, true).await;
    matches(&matcher, Method::PUT, false).await;
    matches(&matcher, Method::DELETE, false).await;
    matches(&matcher, Method::PATCH, false).await;
    matches(&matcher, Method::TRACE, false).await;
    matches(&matcher, Method::OPTIONS, false).await;
}
