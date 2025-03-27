mod identify;
use crate::identify::Identify;
use http::Request;
use satex_core::body::Body;
use satex_core::component::Args;
use satex_layer::make::MakeRouteLayer;
use satex_layer::strip_prefix::{MakeStripPrefixRouteLayer, StripPrefixRouteLayer};
use tower::{Layer, Service};

async fn _layer(layer: StripPrefixRouteLayer, path: &str) {
    let request = Request::builder()
        .uri("/api/v1/resource")
        .body(Body::empty())
        .unwrap();
    let mut service = layer.layer(Identify::new(|request: Request<Body>| {
        request.uri().path() == path
    }));
    let result = service.call(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn make_with_shortcut() {
    let args = Args::shortcut("0");
    let layer = MakeStripPrefixRouteLayer.make(args).unwrap();
    _layer(layer, "/api/v1/resource").await;

    let args = Args::shortcut("1");
    let layer = MakeStripPrefixRouteLayer.make(args).unwrap();
    _layer(layer, "/v1/resource").await;

    let args = Args::shortcut("2");
    let layer = MakeStripPrefixRouteLayer.make(args).unwrap();
    _layer(layer, "/resource").await;

    let args = Args::shortcut("3");
    let layer = MakeStripPrefixRouteLayer.make(args).unwrap();
    _layer(layer, "/").await;

    let args = Args::shortcut("4");
    let layer = MakeStripPrefixRouteLayer.make(args).unwrap();
    _layer(layer, "/").await;
}

#[tokio::test]
async fn make_with_full() {
    let yaml = "level: 0";
    let value = serde_yaml::from_str(yaml).unwrap();
    let args = Args::full(&value);
    let layer = MakeStripPrefixRouteLayer.make(args).unwrap();
    _layer(layer, "/api/v1/resource").await;

    let yaml = "level: 1";
    let value = serde_yaml::from_str(yaml).unwrap();
    let args = Args::full(&value);
    let layer = MakeStripPrefixRouteLayer.make(args).unwrap();
    _layer(layer, "/v1/resource").await;

    let yaml = "level: 2";
    let value = serde_yaml::from_str(yaml).unwrap();
    let args = Args::full(&value);
    let layer = MakeStripPrefixRouteLayer.make(args).unwrap();
    _layer(layer, "/resource").await;

    let yaml = "level: 3";
    let value = serde_yaml::from_str(yaml).unwrap();
    let args = Args::full(&value);
    let layer = MakeStripPrefixRouteLayer.make(args).unwrap();
    _layer(layer, "/").await;

    let yaml = "level: 4";
    let value = serde_yaml::from_str(yaml).unwrap();
    let args = Args::full(&value);
    let layer = MakeStripPrefixRouteLayer.make(args).unwrap();
    _layer(layer, "/").await;
}
