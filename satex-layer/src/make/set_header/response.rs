use satex_core::config::args::Args;
use satex_core::serde::http::{SerdeHeaderName, SerdeHeaderValue};
use satex_core::Error;

use crate::make::set_header::common::{FixedMakeHeaderValue, InsertHeaderMode};
use crate::{MakeRouteServiceLayer, __layer};

type SetResponseHeaderLayer = tower_http::set_header::SetResponseHeaderLayer<FixedMakeHeaderValue>;

__layer! {
    SetResponseHeader,
    name: SerdeHeaderName,
    value: SerdeHeaderValue,
    mode: Option<InsertHeaderMode>,
}

fn make(args: Args) -> Result<SetResponseHeaderLayer, Error> {
    let config = Config::try_from(args)?;
    let make = FixedMakeHeaderValue::new(config.value.into());
    let header_name = config.name.into();
    match config.mode {
        Some(InsertHeaderMode::Append) => Ok(SetResponseHeaderLayer::appending(header_name, make)),
        Some(InsertHeaderMode::IfNotPresent) => {
            Ok(SetResponseHeaderLayer::if_not_present(header_name, make))
        }
        _ => Ok(SetResponseHeaderLayer::overriding(header_name, make)),
    }
}

#[cfg(test)]
mod test {
    use std::convert::Infallible;

    use hyper::{Request, Response};
    use satex_core::{
        config::args::{Args, Shortcut},
        http::Body,
    };
    use tower::{service_fn, Layer, Service};

    use crate::{make::set_header::MakeSetResponseHeaderLayer, MakeRouteServiceLayer};

    #[tokio::test]
    async fn test_layer() {
        let args = Args::Shortcut(Shortcut::new("k1,v1"));
        let make = MakeSetResponseHeaderLayer::default();
        let layer = make.make(args).unwrap();
        let request = Request::new(Body::empty());
        let service = service_fn(|_: Request<Body>| async move {
            Ok::<_, Infallible>(Response::new(Body::empty()))
        });
        let mut service = layer.layer(service);
        let mut response = service.call(request).await.unwrap();
        let data = response
            .headers_mut()
            .remove("k1")
            .map(|value| value.to_str().unwrap().to_string())
            .unwrap_or_default();
        assert_eq!(data, "v1");
    }
}
