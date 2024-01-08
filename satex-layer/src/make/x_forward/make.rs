use satex_core::config::args::Args;
use satex_core::Error;

use crate::__layer;
use crate::make::x_forward::layer::XForwardLayer;
use crate::make::x_forward::Mode;
use crate::make::MakeRouteServiceLayer;

__layer! {
    XForward,
    mode: Option<Mode>
}

fn make(args: Args) -> Result<XForwardLayer, Error> {
    Config::try_from(args).map(|config| XForwardLayer::new(config.mode.unwrap_or_default()))
}

#[cfg(test)]
mod test {
    use std::convert::Infallible;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use bytes::Buf;
    use http_body_util::BodyExt;
    use hyper::header::HeaderValue;
    use hyper::{Request, Response};
    use tower::{service_fn, Layer, Service};

    use satex_core::config::args::{Args, Shortcut};
    use satex_core::essential::Essential;
    use satex_core::http::Body;

    use crate::make::x_forward::{MakeXForwardLayer, FORWARD_NODE_SEP, X_FORWARDED_FOR};
    use crate::MakeRouteServiceLayer;

    #[tokio::test]
    async fn test_append() {
        let args = Args::Shortcut(Shortcut::none());
        let make = MakeXForwardLayer::default();
        let layer = make.make(args).unwrap();
        let service = service_fn(|request: Request<Body>| async move {
            let value = request
                .headers()
                .get(X_FORWARDED_FOR)
                .map(|value| value.to_str().unwrap().to_string())
                .unwrap_or_default();
            Ok::<_, Infallible>(Response::new(Body::from(value)))
        });
        let mut service = layer.layer(service);
        let mut request = Essential::set_extension(
            Request::new(Body::empty()),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 3000),
        );
        request
            .headers_mut()
            .insert(X_FORWARDED_FOR, HeaderValue::from_static("192.168.90.1"));
        let response = service.call(request).await.unwrap();
        let collected = response.into_body().collect().await.unwrap();
        let buf = collected.aggregate();
        let data = String::from_utf8(buf.chunk().to_vec()).unwrap();
        assert_eq!(
            data,
            vec!["127.0.0.1", "192.168.90.1"].join(FORWARD_NODE_SEP)
        );
    }

    #[tokio::test]
    async fn test_override() {
        let args = Args::Shortcut(Shortcut::new("Override"));
        let make = MakeXForwardLayer::default();
        let layer = make.make(args).unwrap();
        let service = service_fn(|request: Request<Body>| async move {
            let value = request
                .headers()
                .get(X_FORWARDED_FOR)
                .map(|value| value.to_str().unwrap().to_string())
                .unwrap_or_default();
            Ok::<_, Infallible>(Response::new(Body::from(value)))
        });
        let mut service = layer.layer(service);
        let mut request = Essential::set_extension(
            Request::new(Body::empty()),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 3000),
        );
        request
            .headers_mut()
            .insert(X_FORWARDED_FOR, HeaderValue::from_static("192.168.90.1"));
        let response = service.call(request).await.unwrap();
        let collected = response.into_body().collect().await.unwrap();
        let buf = collected.aggregate();
        let data = String::from_utf8(buf.chunk().to_vec()).unwrap();
        assert_eq!(data, "127.0.0.1");
    }
}
