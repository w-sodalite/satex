mod identify;

use crate::identify::Identify;
use http::header::HOST;
use http::{HeaderValue, Request};
use satex_core::body::Body;
use satex_core::component::Args;
use satex_layer::make::MakeRouteLayer;
use satex_layer::remove_header::{MakeRemoveRequestHeaderRouteLayer, RemoveRequestHeaderLayer};
use tower::{Layer, Service};

async fn _layer(layer: RemoveRequestHeaderLayer) {
    let mut service = layer.layer(Identify::new(|request: Request<Body>| {
        !request.headers().contains_key(HOST)
    }));
    let mut request = Request::builder().uri("/").body(Body::empty()).unwrap();
    request
        .headers_mut()
        .insert(HOST, HeaderValue::from_static("127.0.0.1:3000"));

    let result = service.call(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn make_with_shortcut() {
    let args = Args::shortcut("host");
    let layer = MakeRemoveRequestHeaderRouteLayer.make(args).unwrap();
    _layer(layer).await;
}

#[tokio::test]
async fn make_with_full() {
    let yaml = "name: host";
    let value = serde_yaml::from_str(yaml).unwrap();
    let args = Args::full(&value);
    let layer = MakeRemoveRequestHeaderRouteLayer.make(args).unwrap();
    _layer(layer).await;
}
