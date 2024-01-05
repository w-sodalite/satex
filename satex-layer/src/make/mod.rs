use std::sync::Arc;

use bytes::Bytes;
use hyper::{Request, Response};
use tower::{Layer, Service};

use satex_core::http::Body;
use satex_core::{make, BoxError};
use satex_service::NamedRouteService;

use crate::NamedRouteServiceLayer;

pub mod concurrency_limit;
pub mod cors;
pub mod default;
pub mod keep_host_header;
pub mod path_strip;
pub mod rate_limit;
pub mod request_body_limit;
pub mod set_header;
pub mod set_status;

make! {
    @compcat,
    MakeRouteServiceLayer,
    Layer,
    (Layer<NamedRouteService>),
    NamedRouteServiceLayer
}

impl ArcMakeRouteServiceLayer {
    pub fn new<M, L, ResBody, E>(make: M) -> Self
    where
        M: MakeRouteServiceLayer<Layer = L> + Send + Sync + 'static,
        L: Layer<NamedRouteService> + Send + Sync + 'static,
        L::Service: Service<Request<Body>, Response = Response<ResBody>, Error = E>
            + Clone
            + Send
            + 'static,
        <<L as Layer<NamedRouteService>>::Service as Service<Request<Body>>>::Future:
            Send + 'static,
        ResBody: hyper::body::Body<Data = Bytes> + Send + 'static,
        ResBody::Error: Into<BoxError> + Send + 'static,
        E: Into<BoxError> + Send + 'static,
    {
        let name = make.name();
        Self(Arc::new(MakeFn::new(make, |inner| {
            NamedRouteServiceLayer::new(name, inner)
        })))
    }
}

///
/// 内部API
///
#[doc(hidden)]
#[macro_export]
macro_rules! __layer {
    ($name:ident $(,)?) => {
        satex_core::make_impl!(MakeRouteServiceLayer,Layer,$name,Default);
    };
    ($name:ident,$($(#[$meta:meta])* $vis:vis $field:ident : $ty:ty),* $(,)?) => {
        satex_core::make_impl!(MakeRouteServiceLayer,Layer,$name,Default,$($(#[$meta])* $vis $field : $ty),*);
    };
    ($name:ident,$mode:ident,$($(#[$meta:meta])* $vis:vis $field:ident : $ty:ty),* $(,)?) => {
        satex_core::make_impl!(MakeRouteServiceLayer,Layer,$name,$mode,$($(#[$meta])* $vis $field : $ty),*);
    };
}
