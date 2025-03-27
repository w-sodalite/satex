mod identify;

use crate::identify::Identify;
use http::Request;
use satex_core::body::Body;
use satex_core::component::Args;
use satex_layer::make::MakeRouteLayer;
use satex_layer::set_prefix::{MakeSetPrefixRouteLayer, SetPrefixLayer};
use tower::{Layer, Service};

async fn _layer(layer: SetPrefixLayer, path: &str) {
    let mut service = layer.layer(Identify::new(|request: Request<Body>| {
        request.uri().path() == path
    }));
    let request = Request::builder()
        .uri("/resource")
        .body(Body::empty())
        .unwrap();
    let result = service.call(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn make_with_shortcut() {
    let args = Args::shortcut("/api");
    let layer = MakeSetPrefixRouteLayer.make(args).unwrap();
    _layer(layer, "/api/resource").await;

    let args = Args::shortcut("/api/v1");
    let layer = MakeSetPrefixRouteLayer.make(args).unwrap();
    _layer(layer, "/api/v1/resource").await;
}

#[tokio::test]
async fn make_with_full() {
    let yaml = "prefix: /api";
    let value = serde_yaml::from_str(yaml).unwrap();
    let args = Args::full(&value);
    let layer = MakeSetPrefixRouteLayer.make(args).unwrap();
    _layer(layer, "/api/resource").await;

    let yaml = "prefix: /api/v1";
    let value = serde_yaml::from_str(yaml).unwrap();
    let args = Args::full(&value);
    let layer = MakeSetPrefixRouteLayer.make(args).unwrap();
    _layer(layer, "/api/v1/resource").await;
}
