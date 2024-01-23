use hyper::StatusCode;
use tower_http::set_status::SetStatusLayer;

use satex_core::config::args::Args;
use satex_core::{satex_error, Error};

use crate::make::make_layer;
use crate::make::MakeRouteServiceLayer;

make_layer! {
    SetStatus,
    #[serde(deserialize_with="satex_core::serde::tot::as_u64")]
    status: u64,
}

fn make(args: Args) -> Result<SetStatusLayer, Error> {
    Config::try_from(args).and_then(|config| {
        StatusCode::from_u16(config.status as u16)
            .map(SetStatusLayer::new)
            .map_err(|e| satex_error!(e))
    })
}

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn test_layer() {
        use std::convert::Infallible;

        use hyper::{Request, Response};
        use satex_core::{
            config::args::{Args, Shortcut},
            http::Body,
        };
        use tower::{service_fn, Layer, Service};

        use crate::{make::set_status::MakeSetStatusLayer, MakeRouteServiceLayer};

        let args = Args::Shortcut(Shortcut::from("404"));
        let make = MakeSetStatusLayer::default();
        let layer = make.make(args).unwrap();
        let service = service_fn(|_: Request<Body>| async move {
            Ok::<_, Infallible>(Response::new(Body::empty()))
        });
        let mut service = layer.layer(service);
        let request = Request::new(Body::empty());
        let response = service.call(request).await.unwrap();
        assert_eq!(response.status().as_u16(), 404);
    }
}
