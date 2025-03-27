mod identify;

use crate::identify::Identify;
use http::{Method, Request};
use satex_core::body::Body;
use satex_core::component::Args;
use satex_layer::make::MakeRouteLayer;
use satex_layer::set_method::MakeSetMethodRouteLayer;
use serde_yaml::Value;
use std::str::FromStr;
use tower::{Layer, Service};

async fn layer_shortcut(method: &str) {
    let args = Args::shortcut(method);
    let layer = MakeSetMethodRouteLayer.make(args).unwrap();

    let method = Method::from_str(method).unwrap();
    let mut service = layer.layer(Identify::new(|request: Request<Body>| {
        request.method() == method
    }));
    let request = Request::new(Body::empty());
    let result = service.call(request).await;
    assert!(result.is_ok());
}

async fn layer_full(method: &str) {
    let yaml = format!("method: {}", method);
    let value = serde_yaml::from_str::<Value>(yaml.as_str()).unwrap();
    let args = Args::full(&value);
    let layer = MakeSetMethodRouteLayer.make(args).unwrap();

    let method = Method::from_str(method).unwrap();
    let mut service = layer.layer(Identify::new(|request: Request<Body>| {
        request.method() == method
    }));
    let request = Request::new(Body::empty());
    let result = service.call(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn make_with_shortcut() {
    layer_shortcut("GET").await;
    layer_shortcut("POST").await;
    layer_shortcut("PUT").await;
    layer_shortcut("DELETE").await;
    layer_shortcut("OPTIONS").await;
    layer_shortcut("TRACE").await;
}

#[tokio::test]
async fn make_with_full() {
    layer_full("GET").await;
    layer_full("POST").await;
    layer_full("PUT").await;
    layer_full("DELETE").await;
    layer_full("OPTIONS").await;
    layer_full("TRACE").await;
}
