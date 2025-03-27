use http::{HeaderValue, Request, Response};
use satex_core::body::Body;
use satex_core::component::Args;
use satex_layer::make::MakeRouteLayer;
use satex_layer::set_header::MakeSetResponseHeaderRouteLayer;
use std::convert::Infallible;
use tower::{service_fn, Layer, Service};
use tower_http::set_header::SetResponseHeaderLayer;

async fn _layer(layer: SetResponseHeaderLayer<HeaderValue>, value: &[&str]) {
    let request = Request::new(Body::empty());
    let service = service_fn(|_| async move {
        let mut response = Response::new(Body::empty());
        response
            .headers_mut()
            .insert("x-app", HeaderValue::from_static("v1"));
        Ok::<_, Infallible>(response)
    });
    let mut service = layer.layer(service);
    let response = service.call(request).await.unwrap();
    let app = response
        .headers()
        .get_all("x-app")
        .into_iter()
        .flat_map(|v| v.to_str())
        .collect::<Vec<_>>();
    assert_eq!(app.as_slice(), value);
}

#[tokio::test]
async fn make_with_shortcut() {
    let args = Args::shortcut("x-app, v2");
    let layer = MakeSetResponseHeaderRouteLayer.make(args).unwrap();
    _layer(layer, &["v2"]).await;

    let args = Args::shortcut("x-app, v2, Appending");
    let layer = MakeSetResponseHeaderRouteLayer.make(args).unwrap();
    _layer(layer, &["v1", "v2"]).await;

    let args = Args::shortcut("x-app, v2, IfNotPresent");
    let layer = MakeSetResponseHeaderRouteLayer.make(args).unwrap();
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
    let layer = MakeSetResponseHeaderRouteLayer.make(args).unwrap();
    _layer(layer, &["v2"]).await;

    let yaml = r#"
        name: x-app
        value: v2
        policy: Appending
    "#;
    let value = serde_yaml::from_str(yaml).unwrap();
    let args = Args::full(&value);
    let layer = MakeSetResponseHeaderRouteLayer.make(args).unwrap();
    _layer(layer, &["v1", "v2"]).await;

    let yaml = r#"
        name: x-app
        value: v2
        policy: IfNotPresent
    "#;
    let value = serde_yaml::from_str(yaml).unwrap();
    let args = Args::full(&value);
    let layer = MakeSetResponseHeaderRouteLayer.make(args).unwrap();
    _layer(layer, &["v1"]).await;
}
