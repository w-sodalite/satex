use http::{Method, Request, StatusCode};
use satex_core::body::Body;
use satex_core::component::Args;
use satex_service::make::MakeRouteService;
use satex_service::status_code::{MakeStatusCodeRouteService, StatusCodeRouteService};
use serde_yaml::Value;
use tower::Service;

async fn call(mut service: StatusCodeRouteService) -> StatusCode {
    let request = Request::builder()
        .uri("/")
        .method(Method::GET)
        .body(Body::empty())
        .unwrap();

    let response = service.call(request).await.unwrap();
    response.status()
}

#[tokio::test]
async fn call_200() {
    let service = StatusCodeRouteService::new(StatusCode::OK);
    let status_code = call(service).await;
    assert_eq!(status_code, StatusCode::OK);
}

#[tokio::test]
async fn call_404() {
    let service = StatusCodeRouteService::new(StatusCode::NOT_FOUND);
    let status_code = call(service).await;
    assert_eq!(status_code, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn make_with_shortcut() {
    let args = Args::shortcut("200");
    let service = MakeStatusCodeRouteService.make(args).unwrap();
    let status_code = call(service).await;
    assert_eq!(status_code, StatusCode::OK);
}

#[tokio::test]
async fn make_with_full() {
    let value = serde_yaml::from_str::<Value>(r#"status: 404"#).unwrap();
    let args = Args::full(&value);
    let service = MakeStatusCodeRouteService.make(args).unwrap();
    let status_code = call(service).await;
    assert_eq!(status_code, StatusCode::NOT_FOUND);
}
