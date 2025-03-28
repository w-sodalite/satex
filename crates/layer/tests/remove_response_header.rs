use http::header::CONTENT_TYPE;
use http::{HeaderValue, Request, Response};
use satex_core::body::Body;
use satex_core::component::Args;
use satex_layer::make::MakeRouteLayer;
use satex_layer::remove_header::{
    MakeRemoveResponseHeaderRouteLayer, RemoveResponseHeaderLayer,
};
use std::convert::Infallible;
use tower::{service_fn, Layer, Service};

async fn _layer(layer: RemoveResponseHeaderLayer) {
    let mut service = layer.layer(service_fn(|_| async move {
        let mut response = Response::new(Body::empty());
        response
            .headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        Ok::<_, Infallible>(response)
    }));
    let request = Request::new(Body::empty());

    let response = service.call(request).await.unwrap();
    assert!(!response.headers().contains_key(CONTENT_TYPE))
}

#[tokio::test]
async fn make_with_shortcut() {
    let args = Args::shortcut("content-type");
    let layer = MakeRemoveResponseHeaderRouteLayer.make(args).unwrap();
    _layer(layer).await;
}

#[tokio::test]
async fn make_with_full() {
    let yaml = "name: content-type";
    let value = serde_yaml::from_str(yaml).unwrap();
    let args = Args::full(&value);
    let layer = MakeRemoveResponseHeaderRouteLayer.make(args).unwrap();
    _layer(layer).await;
}
