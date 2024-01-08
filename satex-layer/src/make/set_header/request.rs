use satex_core::config::args::Args;
use satex_core::serde::http::{SerdeHeaderName, SerdeHeaderValue};
use satex_core::Error;

use crate::make::set_header::common::{FixedMakeHeaderValue, InsertHeaderMode};
use crate::{MakeRouteServiceLayer, __make_layer};

type SetRequestHeaderLayer = tower_http::set_header::SetRequestHeaderLayer<FixedMakeHeaderValue>;

__make_layer! {
    SetRequestHeader,
    name: SerdeHeaderName,
    value: SerdeHeaderValue,
    mode: Option<InsertHeaderMode>,
}

fn make(args: Args) -> Result<SetRequestHeaderLayer, Error> {
    let config = Config::try_from(args)?;
    let make = FixedMakeHeaderValue::new(config.value.into());
    let header_name = config.name.into();
    match config.mode {
        Some(InsertHeaderMode::Append) => Ok(SetRequestHeaderLayer::appending(header_name, make)),
        Some(InsertHeaderMode::IfNotPresent) => {
            Ok(SetRequestHeaderLayer::if_not_present(header_name, make))
        }
        _ => Ok(SetRequestHeaderLayer::overriding(header_name, make)),
    }
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

    use crate::{make::set_header::MakeSetRequestHeaderLayer, MakeRouteServiceLayer};

    #[tokio::test]
    async fn test_layer() {
        let args = Args::Shortcut(Shortcut::from("k1,v1"));
        let make = MakeSetRequestHeaderLayer::default();
        let layer = make.make(args).unwrap();
        let request = Request::new(Body::empty());
        let service = service_fn(|mut request: Request<Body>| async move {
            let value = request
                .headers_mut()
                .remove("k1")
                .map(|value| value.to_str().unwrap().to_string())
                .unwrap_or_default();
            Ok::<_, Infallible>(Response::new(Body::from(value)))
        });
        let mut service = layer.layer(service);
        let response = service.call(request).await.unwrap();
        let collected = response.into_body().collect().await.unwrap();
        let buf = collected.aggregate();
        let data = String::from_utf8(buf.chunk().to_vec()).unwrap();
        assert_eq!(data, "v1");
    }
}
