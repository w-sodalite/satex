mod identify;

use crate::identify::Identify;
use http::{HeaderValue, Request};
use satex_core::body::Body;
use satex_core::component::Args;
use satex_layer::make::MakeRouteLayer;
use satex_layer::set_header::MakeSetRequestHeaderRouteLayer;
use tower::{Layer, Service};
use tower_http::set_header::SetRequestHeaderLayer;

async fn _layer(layer: SetRequestHeaderLayer<HeaderValue>, value: &[&str]) {
    let mut request = Request::new(Body::empty());
    request
        .headers_mut()
        .insert("x-app", HeaderValue::from_static("v1"));
    let service = Identify::new(|request: Request<Body>| {
        let app = request
            .headers()
            .get_all("x-app")
            .into_iter()
            .flat_map(|x| x.to_str())
            .collect::<Vec<_>>();
        app.as_slice() == value
    });
    let mut service = layer.layer(service);
    let result = service.call(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn make_with_shortcut() {
    let args = Args::shortcut("x-app, v2");
    let layer = MakeSetRequestHeaderRouteLayer.make(args).unwrap();
    _layer(layer, &["v2"]).await;

    let args = Args::shortcut("x-app, v2, Appending");
    let layer = MakeSetRequestHeaderRouteLayer.make(args).unwrap();
    _layer(layer, &["v1", "v2"]).await;

    let args = Args::shortcut("x-app, v2, IfNotPresent");
    let layer = MakeSetRequestHeaderRouteLayer.make(args).unwrap();
    _layer(layer, &["v1"]).await;
}

#[tokio::test]
async fn make_with_full() {
    let yaml = r#"
        name: x-app
        value: v2
    "#;
    let value = serde_yaml::from_str(yaml).unwrap();
    let args = Args::full(&value);
    let layer = MakeSetRequestHeaderRouteLayer.make(args).unwrap();
    _layer(layer, &["v2"]).await;

    let yaml = r#"
        name: x-app
        value: v2
        policy: Appending
    "#;
    let value = serde_yaml::from_str(yaml).unwrap();
    let args = Args::full(&value);
    let layer = MakeSetRequestHeaderRouteLayer.make(args).unwrap();
    _layer(layer, &["v1", "v2"]).await;

    let yaml = r#"
        name: x-app
        value: v2
        policy: IfNotPresent
    "#;
    let value = serde_yaml::from_str(yaml).unwrap();
    let args = Args::full(&value);
    let layer = MakeSetRequestHeaderRouteLayer.make(args).unwrap();
    _layer(layer, &["v1"]).await;
}
