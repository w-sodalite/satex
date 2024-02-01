use satex_core::config::args::Args;
use satex_core::Error;

use crate::make::make_layer;
use crate::make::set_mode::SetMode;
use crate::make::set_query::layer::SetQueryLayer;
use crate::MakeRouteServiceLayer;

make_layer! {
    SetQuery,
    name: String,
    value: String,
    #[serde(default)]
    mode: SetMode
}

fn make(args: Args) -> Result<SetQueryLayer, Error> {
    let config = Config::try_from(args)?;
    Ok(SetQueryLayer::new(config.name, config.value, config.mode))
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
        http::Body,
    };

    use crate::make::set_query::make::MakeSetQueryLayer;
    use crate::MakeRouteServiceLayer;

    #[tokio::test]
    async fn test_layer() {
        let args = Args::Shortcut(Shortcut::from("k1,v1"));
        let make = MakeSetQueryLayer::default();
        let layer = make.make(args).unwrap();
        let request = Request::new(Body::empty());
        let service = service_fn(|mut request: Request<Body>| async move {
            let query = request.uri().query().unwrap_or("").to_string();
            Ok::<_, Infallible>(Response::new(Body::from(query)))
        });
        let mut service = layer.layer(service);
        let response = service.call(request).await.unwrap();
        let collected = response.into_body().collect().await.unwrap();
        let buf = collected.aggregate();
        let data = String::from_utf8(buf.chunk().to_vec()).unwrap();
        assert_eq!(data, "k1=v1");
    }
}
