use bytes::Bytes;
use http::Request;
use http_body_util::BodyExt;
use satex_core::body::Body;
use satex_core::component::Args;
use satex_service::echo::{EchoRouteService, MakeEchoRouteService};
use satex_service::make::MakeRouteService;
use serde_yaml::Value;
use tower::Service;

async fn call(mut service: EchoRouteService) -> Bytes {
    let request = Request::new(Body::empty());
    let response = service.call(request).await.unwrap();
    let collected = response.into_body().collect().await.unwrap();
    collected.to_bytes()
}

#[tokio::test]
async fn call_respond_empty() {
    let service = EchoRouteService::default();
    let bytes = call(service).await;
    assert_eq!(&bytes[..], b"")
}

#[tokio::test]
async fn call_respond_body() {
    let service = EchoRouteService::new("Hello World!");
    let bytes = call(service).await;
    assert_eq!(&bytes[..], b"Hello World!")
}

#[tokio::test]
async fn make_with_shortcut() {
    let args = Args::shortcut("Hello World!");
    let service = MakeEchoRouteService.make(args).unwrap();
    let bytes = call(service).await;
    assert_eq!(&bytes[..], b"Hello World!")
}

#[tokio::test]
async fn make_with_full() {
    let value = serde_yaml::from_str::<Value>(r#"text: "Hello World!""#).unwrap();
    let args = Args::full(&value);
    let service = MakeEchoRouteService.make(args).unwrap();
    let bytes = call(service).await;
    assert_eq!(&bytes[..], b"Hello World!")
}
