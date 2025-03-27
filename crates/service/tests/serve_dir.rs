mod util;

use crate::util::test_file_dir;
use http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use satex_core::body::Body;
use satex_core::component::Args;
use satex_service::make::MakeRouteService;
use satex_service::serve_dir::MakeServeDirRouteService;
use serde_yaml::Value;
use tower::Service;
use tower_http::services::ServeDir;

const TEXT: &str = include_str!("file/hello.txt");

async fn call(mut service: ServeDir) {
    let request = Request::builder()
        .uri("/hello.txt")
        .method(Method::GET)
        .body(Body::empty())
        .unwrap();

    let response = service.call(request).await.unwrap();
    let status_code = response.status();
    assert_eq!(status_code, StatusCode::OK);

    let body = response.into_body().collect().await.unwrap();
    let bytes = body.to_bytes();
    assert_eq!(&bytes[..], TEXT.as_bytes());
}

#[tokio::test]
async fn call_file() {
    let service = ServeDir::new(test_file_dir());
    call(service).await
}

#[tokio::test]
async fn make_with_shortcut() {
    let path = format!("{}", test_file_dir().display());
    let args = Args::shortcut(&path);
    let service = MakeServeDirRouteService.make(args).unwrap();
    call(service).await
}

#[tokio::test]
async fn make_with_full() {
    let path = format!("{}", test_file_dir().display());
    let value = serde_yaml::from_str::<Value>(&format!(r#"path: {}"#, path)).unwrap();
    let args = Args::full(&value);
    let service = MakeServeDirRouteService.make(args).unwrap();
    call(service).await
}
