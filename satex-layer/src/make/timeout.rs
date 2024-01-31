use std::time::Duration;

use tower_http::timeout::TimeoutLayer;

use satex_core::config::args::Args;
use satex_core::Error;

use crate::make::make_layer;
use crate::MakeRouteServiceLayer;

make_layer! {
    Timeout,

    #[serde(deserialize_with="satex_core::serde::tot::as_u64")]
    secs: u64
}

fn make(args: Args) -> Result<TimeoutLayer, Error> {
    let config = Config::try_from(args)?;
    Ok(TimeoutLayer::new(Duration::from_secs(config.secs)))
}

#[cfg(test)]
mod test {
    use std::convert::Infallible;
    use std::time::Duration;

    use hyper::{Request, Response};
    use tokio::time::sleep;
    use tower::{service_fn, Layer, Service};

    use satex_core::config::args::{Args, Shortcut};
    use satex_core::http::Body;

    use crate::make::timeout::MakeTimeoutLayer;
    use crate::MakeRouteServiceLayer;

    #[tokio::test]
    async fn test_layer() {
        let args = Args::from(Shortcut::from("3"));
        let make = MakeTimeoutLayer::default();
        let layer = make.make(args).unwrap();
        let service = service_fn(|_: Request<Body>| async move {
            sleep(Duration::from_secs(5)).await;
            Ok::<_, Infallible>(Response::new(Body::empty()))
        });
        let mut service = layer.layer(service);
        let request = Request::new(Body::empty());
        let response = service.call(request).await.unwrap();
        assert_eq!(response.status().as_u16(), 408);
    }
}
