use satex_core::config::args::Args;
use satex_core::Error;

use crate::make::keep_host_header::layer::KeepHostHeaderLayer;
use crate::make::make_layer;
use crate::MakeRouteServiceLayer;

make_layer!(KeepHostHeader);

fn make(_: Args) -> Result<KeepHostHeaderLayer, Error> {
    Ok(KeepHostHeaderLayer::default())
}

#[cfg(test)]
mod test {
    use std::convert::Infallible;

    use bytes::Buf;
    use http_body_util::BodyExt;
    use hyper::{Request, Response};
    use tower::{service_fn, Layer, Service};

    use satex_core::{
        config::args::{Args, Shortcut},
        essential::Essential,
        http::Body,
    };
    use satex_service::KeepHostHeaderState;

    use crate::MakeRouteServiceLayer;

    use super::MakeKeepHostHeaderLayer;

    #[tokio::test]
    async fn test_layer() {
        let args = Args::Shortcut(Shortcut::none());
        let make = MakeKeepHostHeaderLayer::default();
        let layer = make.make(args).unwrap();
        let service = service_fn(|mut request: Request<Body>| async move {
            Ok::<_, Infallible>(Response::new(Body::from(format!(
                "{}",
                request
                    .extensions_mut()
                    .remove::<KeepHostHeaderState>()
                    .is_some()
            ))))
        });
        let mut service = layer.layer(service);
        let request = Essential::attach(
            Request::new(Body::empty()),
            "127.0.0.1:3000".parse().unwrap(),
        );
        let response = service.call(request).await.unwrap();
        let body = response.into_body();
        let collected = body.collect().await.unwrap();
        let buf = collected.aggregate();
        let data = String::from_utf8(buf.chunk().to_vec()).unwrap();
        let flag = data.parse::<bool>().unwrap();
        assert!(flag)
    }
}
