use satex_core::{config::args::Args, Error};

use crate::{MakeRouteServiceLayer, __make_layer};

use super::layer::SetPathLayer;

__make_layer! {
    SetPath,
    path:String
}

fn make(args: Args) -> Result<SetPathLayer, Error> {
    let config = Config::try_from(args)?;
    Ok(SetPathLayer::new(config.path))
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

    use crate::{make::set_path::MakeSetPathLayer, MakeRouteServiceLayer};

    #[tokio::test]
    async fn test_layer() {
        let args = Args::Shortcut(Shortcut::from("/a/b/index.html"));
        let make = MakeSetPathLayer::default();
        let layer = make.make(args).unwrap();
        let service = service_fn(|request: Request<Body>| async move {
            Ok::<_, Infallible>(Response::new(Body::from(request.uri().to_string())))
        });
        let mut service = layer.layer(service);
        let request = Request::builder()
            .uri("https://www.rust-lang.org/index.html")
            .body(Body::empty())
            .unwrap();
        let response = service.call(request).await.unwrap();
        let collected = response.into_body().collect().await.unwrap();
        let buf = collected.aggregate();
        let data = String::from_utf8(buf.chunk().to_vec()).unwrap();
        assert_eq!(data, "https://www.rust-lang.org/a/b/index.html")
    }
}
