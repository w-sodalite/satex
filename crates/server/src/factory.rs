use actix_service::{Service as ActixService, ServiceFactory};
use actix_tls::accept::rustls_0_23::TlsStream;
use futures::future::LocalBoxFuture;
use http::Response;
use hyper::body::Incoming;
use hyper::service::{service_fn, Service as HyperService};
use hyper::Request;
use hyper_util::rt::TokioIo;
use hyper_util::server::conn::auto::Builder as ConnectorBuilder;
use satex_core::executor::SpawnLocalExecutor;
use satex_core::extension::ClientAddr;
use satex_core::util::try_downcast;
use satex_core::{BoxError, Error};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio_util::either::Either;
use tracing::debug;

pub(crate) struct HttpServiceFactory<M> {
    make_service: M,
    builder: ConnectorBuilder<SpawnLocalExecutor>,
}

impl<M> HttpServiceFactory<M> {
    pub fn new(make_service: M) -> Self {
        Self {
            make_service,
            builder: ConnectorBuilder::new(SpawnLocalExecutor::new()),
        }
    }
}

impl<A, M, S, ResBody> ServiceFactory<A> for HttpServiceFactory<M>
where
    A: AsyncRead + AsyncWrite + Send + Unpin + 'static,
    M: HyperService<(), Response=S, Error=()> + Clone + 'static,
    S: HyperService<Request<Incoming>, Response=Response<ResBody>> + Clone + 'static,
    S::Error: Into<BoxError>,
    ResBody: http_body::Body + 'static,
    ResBody::Error: Into<BoxError>,
{
    type Response = ();
    type Error = Error;
    type Config = ();
    type Service = HttpService<S>;
    type InitError = ();
    type Future = LocalBoxFuture<'static, Result<Self::Service, Self::InitError>>;

    fn new_service(&self, _: Self::Config) -> Self::Future {
        let make_service = self.make_service.clone();
        let builder = self.builder.clone();
        Box::pin(async move {
            make_service
                .call(())
                .await
                .map(|service| HttpService { service, builder })
        })
    }
}

pub(crate) struct HttpService<S> {
    service: S,
    builder: ConnectorBuilder<SpawnLocalExecutor>,
}

impl<A, S, ResBody> ActixService<A> for HttpService<S>
where
    A: AsyncRead + AsyncWrite + Send + Unpin + 'static,
    S: HyperService<Request<Incoming>, Response=Response<ResBody>> + Clone + 'static,
    S::Error: Into<BoxError>,
    ResBody: http_body::Body + 'static,
    ResBody::Error: Into<BoxError>,
{
    type Response = ();
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, stream: A) -> Self::Future {
        let (client_addr, stream) = match try_downcast::<TcpStream, _>(stream) {
            Ok(stream) => (get_client_addr(&stream), Either::Left(stream)),
            Err(stream) => match try_downcast::<TlsStream<TcpStream>, _>(stream) {
                Ok(stream) => (get_client_addr(stream.get_ref().0), Either::Right(stream)),
                Err(_) => unreachable!(),
            },
        };

        let service = self.service.clone();
        let builder = self.builder.clone();
        Box::pin(async move {
            builder
                .serve_connection_with_upgrades(
                    TokioIo::new(stream),
                    service_fn(move |mut request: Request<Incoming>| {
                        debug!("client ({:?}) request:\n{:#?}", client_addr, request);
                        request
                            .extensions_mut()
                            .insert(ClientAddr::new(client_addr));
                        service.call(request)
                    }),
                )
                .await
                .map_err(Error::new)
        })
    }
}

///
/// 从TcpStream中获取客户端地址
///
/// # Arguments
///
/// * `stream`: Tcp流
///
/// returns: SocketAddr
///
fn get_client_addr(stream: &TcpStream) -> SocketAddr {
    stream
        .peer_addr()
        .unwrap_or_else(|_| SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)))
}
