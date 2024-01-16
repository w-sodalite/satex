use std::fs::File;
use std::future::Future;
use std::io::BufReader;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{ready, Context, Poll};

use bytes::Bytes;
use futures_util::future::{select_all, BoxFuture, SelectAll};
use hyper::{Request, Response};
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use pin_project_lite::pin_project;
use rustls_pki_types::PrivateKeyDer;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpListener;
use tokio_rustls::rustls::pki_types::CertificateDer;
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::TlsAcceptor;
use tower::Service;
use tracing::{info, warn};

use satex_core::config::Tls;
use satex_core::essential::Essential;
use satex_core::http::Body;
use satex_core::task::spawn;
use satex_core::task::JoinHandle;
use satex_core::BoxError;
use satex_core::{satex_error, Error};

use crate::router::{Router, RouterFuture};

pin_project! {
    #[project=ServeProj]
    #[project_replace=ServeProjReplace]
    pub enum Serve {
        Binding {
            bind_addr: SocketAddr,
            router: Router,
            tls: Option<Tls>,
            #[pin]
            future: BoxFuture<'static, Result<TcpListener, std::io::Error>>,
        },
        Listening,
        Accepting{
            bind_addr: SocketAddr,
            #[pin]
            handle: JoinHandle<Result<(),Error>>
        }
    }
}

impl Serve {
    pub fn new(bind_addr: SocketAddr, router: Router, tls: Option<Tls>) -> Self {
        Serve::Binding {
            bind_addr,
            router,
            tls,
            future: Box::pin(TcpListener::bind(bind_addr)),
        }
    }
}

impl Future for Serve {
    type Output = Result<(), Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(loop {
            match self.as_mut().project() {
                ServeProj::Binding {
                    future, bind_addr, ..
                } => {
                    let bind_addr = *bind_addr;
                    match ready!(future.poll(cx)) {
                        Ok(listener) => match self.as_mut().project_replace(Serve::Listening) {
                            ServeProjReplace::Binding { router, tls, .. } => {
                                let handle = match tls {
                                    Some(tls) => {
                                        info!("App serve listen on: (https) - [{:?}]", bind_addr);
                                        match EventLoop::tls_boos(listener, router, tls) {
                                            Ok(boss) => spawn(boss),
                                            Err(e) => break Err(satex_error!(e)),
                                        }
                                    }
                                    None => {
                                        info!("App serve listen on: (http) - [{:?}]", bind_addr);
                                        spawn(EventLoop::boss(listener, router))
                                    }
                                };
                                self.as_mut()
                                    .project_replace(Serve::Accepting { handle, bind_addr });
                            }
                            _ => unreachable!(),
                        },
                        Err(e) => {
                            break Err(satex_error!(
                                "App serve listen on [{}] error: {}",
                                bind_addr,
                                e
                            ))
                        }
                    }
                }

                ServeProj::Accepting {
                    handle, bind_addr, ..
                } => match ready!(handle.poll(cx)) {
                    Ok(x) => break x,
                    Err(e) => {
                        break Err(satex_error!(
                            "App serve listen [{:?}] task interrupt: {}",
                            bind_addr,
                            e
                        ))
                    }
                },

                ServeProj::Listening => unreachable!(),
            }
        })
    }
}

pin_project! {
    struct EventLoop {
        #[pin]
        future: BoxFuture<'static, Result<() , Error>>,
    }
}

impl EventLoop {
    fn load_certs(path: &str) -> Result<Vec<CertificateDer<'static>>, Error> {
        let file = File::open(path).map_err(|e| satex_error!(e))?;
        let mut reader = BufReader::new(file);
        rustls_pemfile::certs(&mut reader)
            .map(|x| x.map_err(|e| satex_error!(e)))
            .collect()
    }

    fn load_private_key(path: &str) -> Result<PrivateKeyDer<'static>, Error> {
        let file = File::open(path).map_err(|e| satex_error!(e))?;
        let mut reader = BufReader::new(file);
        rustls_pemfile::private_key(&mut reader)
            .map_err(|e| satex_error!(e))
            .and_then(|key| key.ok_or_else(|| satex_error!("App serve load private key error!")))
    }

    pub fn tls_boos(listener: TcpListener, router: Router, tls: Tls) -> Result<Self, Error> {
        let certs = Self::load_certs(tls.certs())?;
        let private_key = Self::load_private_key(tls.private_key())?;
        let mut tls_config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, private_key)
            .map_err(|e| satex_error!(e))?;
        tls_config.alpn_protocols = tls.alpn_protocols();
        let acceptor = TlsAcceptor::from(Arc::new(tls_config));
        Ok(Self {
            future: Box::pin(async move {
                loop {
                    let router = router.clone();
                    let acceptor = acceptor.clone();
                    match listener.accept().await {
                        Ok((stream, client_addr)) => {
                            info!("App serve listener accept client: {}", client_addr);
                            match acceptor.accept(stream).await {
                                Ok(tls_stream) => {
                                    let worker =
                                        Self::worker(TokioIo::new(tls_stream), router, client_addr);
                                    spawn(worker);
                                }
                                Err(e) => {
                                    warn!("App serve listener accept TLS stream error: {}", e);
                                }
                            };
                        }
                        Err(e) => {
                            warn!("App serve listener accept client error: {}", e);
                        }
                    }
                }
            }),
        })
    }

    pub fn boss(listener: TcpListener, router: Router) -> Self {
        Self {
            future: Box::pin(async move {
                loop {
                    let router = router.clone();
                    match listener.accept().await {
                        Ok((stream, client_addr)) => {
                            info!("App serve listener accept client: {}", client_addr);
                            let worker = Self::worker(TokioIo::new(stream), router, client_addr);
                            spawn(worker);
                        }
                        Err(e) => {
                            warn!("App serve listener accept client error: {}", e);
                        }
                    }
                }
            }),
        }
    }

    pub fn worker<S>(io: TokioIo<S>, router: Router, client_addr: SocketAddr) -> Self
    where
        S: AsyncRead + AsyncWrite + Send + Unpin + 'static,
    {
        Self {
            future: Box::pin(async move {
                Builder::new(TokioExecutor::new())
                    .serve_connection_with_upgrades(io, WorkService::new(router, client_addr))
                    .await
                    .map_err(|e| satex_error!(e))
            }),
        }
    }
}

impl Future for EventLoop {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().future.poll(cx)
    }
}

struct WorkService {
    router: Router,
    client_addr: SocketAddr,
}

impl WorkService {
    pub fn new(router: Router, client_addr: SocketAddr) -> Self {
        Self {
            router,
            client_addr,
        }
    }
}

impl<ReqBody> hyper::service::Service<Request<ReqBody>> for WorkService
where
    ReqBody: hyper::body::Body<Data = Bytes> + Send + 'static,
    ReqBody::Error: Into<BoxError>,
{
    type Response = Response<Body>;
    type Error = Error;
    type Future = RouterFuture<Self::Response, Self::Error>;

    fn call(&self, req: Request<ReqBody>) -> Self::Future {
        let mut router = self.router.clone();
        let req = Essential::attach(req, self.client_addr);
        router.call(req.map(Body::new))
    }
}

pin_project! {

    pub struct Serves {
        #[pin]
        serves: SelectAll<Serve>,
    }

}

impl Serves {
    pub fn new(serves: Vec<Serve>) -> Self {
        Self {
            serves: select_all(serves),
        }
    }
}

impl Future for Serves {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let projection = self.project();
        match projection.serves.poll(cx) {
            Poll::Ready((Ok(_), _, _)) => Poll::Ready(Ok(())),
            Poll::Ready((Err(e), _, _)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }
}
