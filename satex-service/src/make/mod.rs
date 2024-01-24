use std::sync::Arc;

use bytes::Bytes;
use hyper::{Request, Response};
use tower::Service;

use satex_core::http::Body;
use satex_core::{make, BoxError};

use crate::NamedRouteService;

pub mod echo;
pub mod proxy;
pub mod r#static;

make! {
    @compcat,
    MakeRouteService,
    Service,
    (Service<Request<Body>>),
    NamedRouteService
}

impl ArcMakeRouteService {
    pub fn new<M, S, ResBody, E>(make: M) -> Self
    where
        M: MakeRouteService<Service = S> + Send + Sync + 'static,
        S: Service<Request<Body>, Response = Response<ResBody>, Error = E> + Clone + Send + 'static,
        S::Future: Send + 'static,
        ResBody: hyper::body::Body<Data = Bytes> + Send + 'static,
        ResBody::Error: Into<BoxError> + Send + 'static,
        E: Into<BoxError> + Send + 'static,
    {
        let name = make.name();
        Self(Arc::new(MakeFn::new(make, |inner| {
            NamedRouteService::new(name, inner)
        })))
    }
}

///
/// 内部API
///
macro_rules! make_service {
    ($name:ident $(,)?) => {
        satex_core::make_impl!(MakeRouteService,Service,$name);
    };
    ($name:ident,$($(#[$meta:meta])* $vis:vis $field:ident : $ty:ty),* $(,)?) => {
        satex_core::make_impl!(MakeRouteService,Service,$name,Default,$($(#[$meta])* $vis $field : $ty),*);
    };
    ($name:ident,$mode:ident,$($(#[$meta:meta])* $vis:vis $field:ident : $ty:ty),* $(,)?) => {
        satex_core::make_impl!(MakeRouteService,Service,$name,$mode,$($(#[$meta])* $vis $field : $ty),*);
    };
}

pub(crate) use make_service;
