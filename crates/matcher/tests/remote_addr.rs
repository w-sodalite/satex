mod util;
use crate::util::parts;
use http::request::Parts;
use http::Method;
use satex_core::component::Args;
use satex_core::extension::ClientAddr;
use satex_matcher::make::MakeRouteMatcher;
use satex_matcher::remote_addr::MakeRemoteAddrRouteMatcher;
use satex_matcher::RouteMatcher;
use serde_yaml::Value;
use std::net::SocketAddr;

async fn matches_shortcut(parts: &mut Parts, shortcut: &str) {
    let args = Args::shortcut(shortcut);
    let matcher = MakeRemoteAddrRouteMatcher.make(args).unwrap();
    let flag = matcher.matches(parts).await.unwrap();
    assert!(flag);
}

async fn matches_full(parts: &mut Parts, value: &str) {
    let yaml = format!(
        r#"
           policy: Accept
           addrs:
             - {}
        "#,
        value
    );
    let value = serde_yaml::from_str::<Value>(&yaml).unwrap();
    let args = Args::full(&value);
    let matcher = MakeRemoteAddrRouteMatcher.make(args).unwrap();
    let flag = matcher.matches(parts).await.unwrap();
    assert!(flag);
}

#[tokio::test]
async fn make_with_shortcut() {
    let mut parts = parts("/", Method::GET);
    parts.extensions.insert(ClientAddr::new(
        "127.0.0.1:34567".parse::<SocketAddr>().unwrap(),
    ));

    matches_shortcut(&mut parts, "Accept, 127.0.0.1/32").await;
    matches_shortcut(&mut parts, "Accept, 127.0.0.2/24").await;
    matches_shortcut(&mut parts, "Accept, 127.0.1.2/16").await;
    matches_shortcut(&mut parts, "Accept, 127.1.1.2/8").await;
    matches_shortcut(&mut parts, "Accept, 128.1.1.2/0").await;
}

#[tokio::test]
async fn make_with_full() {
    let mut parts = parts("/", Method::GET);
    parts.extensions.insert(ClientAddr::new(
        "127.0.0.1:34567".parse::<SocketAddr>().unwrap(),
    ));

    matches_full(&mut parts, "127.0.0.1/32").await;
    matches_full(&mut parts, "127.0.0.2/24").await;
    matches_full(&mut parts, "127.0.1.2/16").await;
    matches_full(&mut parts, "127.1.1.2/8").await;
    matches_full(&mut parts, "128.1.1.2/0").await;
}
