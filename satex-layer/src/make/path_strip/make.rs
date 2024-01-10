use satex_core::config::args::Args;
use satex_core::Error;

use crate::make::path_strip::layer::PathStripLayer;
use crate::{MakeRouteServiceLayer, __make_layer};

__make_layer! {
    PathStrip,
    #[serde(deserialize_with = "satex_core::serde::tot::as_u64")]
    level: u64,
}

fn make(args: Args) -> Result<PathStripLayer, Error> {
    Config::try_from(args).map(|config| PathStripLayer::new(config.level as usize))
}

#[cfg(test)]
mod test {
    use std::convert::Infallible;

    use bytes::Buf;
    use http_body_util::BodyExt;
    use hyper::{Request, Response};
    use satex_core::{
        config::args::{Args, Shortcut},
        http::Body,
    };
    use tower::{service_fn, Layer, Service};

    use crate::MakeRouteServiceLayer;

    use super::MakePathStripLayer;

    #[tokio::test]
    async fn test_layer() {
        let args = Args::Shortcut(Shortcut::from("1"));
        let make = MakePathStripLayer::default();
        let layer = make.make(args).unwrap();
        let request = Request::builder()
            .uri("https://www.rust-lang.org/a/b/c")
            .body(Body::empty())
            .unwrap();
        let service = service_fn(|request: Request<Body>| async move {
            Ok::<_, Infallible>(
                Response::builder()
                    .body(Body::from(request.uri().to_string()))
                    .unwrap(),
            )
        });
        let mut service = layer.layer(service);
        let response = service.call(request).await.unwrap();
        let body = response.into_body();
        let collected = body.collect().await.unwrap();
        let buf = collected.aggregate();
        let data = String::from_utf8(buf.chunk().to_vec()).unwrap();
        assert_eq!(data, "https://www.rust-lang.org/b/c");
    }
}
