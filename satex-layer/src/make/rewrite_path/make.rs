use satex_core::{config::args::Args, Error};

use crate::{MakeRouteServiceLayer, __make_layer};

use super::layer::RewritePathLayer;

__make_layer! {
    RewritePath,
    path:String
}

fn make(args: Args) -> Result<RewritePathLayer, Error> {
    let config = Config::try_from(args)?;
    Ok(RewritePathLayer::new(config.path))
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::convert::Infallible;

    use bytes::Buf;
    use http_body_util::BodyExt;
    use hyper::{Request, Response};
    use tower::{service_fn, Layer, Service};

    use satex_core::essential::PathVariables;
    use satex_core::{
        config::args::{Args, Shortcut},
        http::Body,
    };

    use crate::make::rewrite_path::make::MakeRewritePathLayer;
    use crate::MakeRouteServiceLayer;

    #[tokio::test]
    async fn test_layer() {
        let args = Args::Shortcut(Shortcut::from("/{prefix}/index.html"));
        let make = MakeRewritePathLayer::default();
        let layer = make.make(args).unwrap();
        let service = service_fn(|request: Request<Body>| async move {
            Ok::<_, Infallible>(Response::new(Body::from(request.uri().to_string())))
        });
        let mut service = layer.layer(service);
        let mut request = Request::builder()
            .uri("https://www.rust-lang.org/index.html")
            .body(Body::empty())
            .unwrap();
        let mut variables = HashMap::new();
        variables.insert("prefix".to_string(), "a/b".to_string());
        request.extensions_mut().insert(PathVariables(variables));
        let response = service.call(request).await.unwrap();
        let collected = response.into_body().collect().await.unwrap();
        let buf = collected.aggregate();
        let data = String::from_utf8(buf.chunk().to_vec()).unwrap();
        assert_eq!(data, "https://www.rust-lang.org/a/b/index.html")
    }
}