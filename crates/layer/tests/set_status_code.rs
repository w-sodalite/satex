mod identify;

use http::{Request, StatusCode};
use identify::Identify;
use satex_core::body::Body;
use satex_core::component::Args;
use satex_layer::make::MakeRouteLayer;
use satex_layer::set_status_code::MakeSetStatusCodeRouteLayer;
use serde_yaml::Value;
use tower::{Layer, Service};
use tower_http::set_status::SetStatusLayer;

async fn _layer(layer: SetStatusLayer) {
    let mut service = layer.layer(Identify::new(|_| true));
    let request = Request::new(Body::empty());
    let response = service.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn make_with_shortcut() {
    let args = Args::shortcut("200");
    let layer = MakeSetStatusCodeRouteLayer.make(args).unwrap();
    _layer(layer).await;
}

#[tokio::test]
async fn make_with_full() {
    let yaml = "status: 200";
    let value = serde_yaml::from_str::<Value>(yaml).unwrap();
    let args = Args::full(&value);
    let layer = MakeSetStatusCodeRouteLayer.make(args).unwrap();
    _layer(layer).await;
}
