use satex_core::config::args::Args;
use satex_core::Error;

use crate::make::keep_host_header::layer::KeepHostHeaderLayer;
use crate::{MakeRouteServiceLayer, __layer};

__layer!(KeepHostHeader);

fn make(_: Args) -> Result<KeepHostHeaderLayer, Error> {
    Ok(KeepHostHeaderLayer::default())
}

#[cfg(test)]
mod test {
    use std::{
        convert::Infallible,
        net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    };

    use bytes::Buf;
    use http_body_util::BodyExt;
    use hyper::{Request, Response};
    use satex_core::{
        config::args::{Args, Shortcut},
        essential::Essential,
        http::Body,
    };
    use tower::{service_fn, Layer, Service};

    use crate::MakeRouteServiceLayer;

    use super::MakeKeepHostHeaderLayer;

    #[tokio::test]
    async fn test_layer() {
        let request = Request::new(Body::empty());
        let (parts, body) = request.into_parts();
        let essential = Essential::new(
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 80)),
            parts.clone(),
        );
        let service = service_fn(|mut request: Request<Body>| async move {
            let essential = request.extensions_mut().remove::<Essential>().unwrap();
            Ok::<_, Infallible>(Response::new(Body::from(format!(
                "{}",
                essential.keep_host_header().is_some()
            ))))
        });
        let mut request = Request::from_parts(parts, body);
        request.extensions_mut().insert(essential);
        let args = Args::Shortcut(Shortcut::none());
        let make = MakeKeepHostHeaderLayer::default();
        let layer = make.make(args).unwrap();
        let mut service = layer.layer(service);
        let response = service.call(request).await.unwrap();
        let body = response.into_body();
        let collected = body.collect().await.unwrap();
        let buf = collected.aggregate();
        let data = String::from_utf8(buf.chunk().to_vec()).unwrap();
        let flag = data.parse::<bool>().unwrap();
        assert!(flag)
    }
}
