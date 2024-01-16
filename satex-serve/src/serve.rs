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

use satex_core::config::{ServeConfig, Tls};
use satex_core::essential::Essential;
use satex_core::http::Body;
use satex_core::task::spawn;
use satex_core::task::JoinHandle;
use satex_core::BoxError;
use satex_core::{satex_error, Error};

use crate::router::make::MakeRouter;
use crate::router::{Router, RouterFuture};

pin_project! {
    #[project=ServeProj]
    #[project_replace=ServeProjReplace]
    pub enum Serve {
        Binding {
            config: ServeConfig,
            #[pin]
            future: BoxFuture<'static, Result<TcpListener, std::io::Error>>,
        },
        Listening,
        Accepting{
            config: ServeConfig,
            #[pin]
            handle: JoinHandle<Result<(),Error>>
        }
    }
}

impl Serve {
    pub fn new(config: ServeConfig) -> Self {
        Serve::Binding {
            future: Box::pin(TcpListener::bind(config.server().bind_addr())),
            config,
        }
    }
}

impl Future for Serve {
    type Output = Result<(), Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(loop {
            match self.as_mut().project() {
                ServeProj::Binding { future, config, .. } => match ready!(future.poll(cx)) {
                    Ok(listener) => match self.as_mut().project_replace(Serve::Listening) {
                        ServeProjReplace::Binding { config, .. } => {
                            match MakeRouter::make(&config) {
                                Ok(router) => {
                                    let handle = match config.server().tls() {
                                        Some(tls) => {
                                            info!(
                                                "Serve [{}] create listener success!",
                                                config.id()
                                            );
                                            match EventLoop::tls_boos(
                                                listener,
                                                router,
                                                tls,
                                                config.id().to_string(),
                                            ) {
                                                Ok(boss) => spawn(boss),
                                                Err(e) => break Err(satex_error!(e)),
                                            }
                                        }
                                        None => {
                                            info!(
                                                "Serve [{}] create listener success!",
                                                config.id()
                                            );
                                            spawn(EventLoop::boss(
                                                listener,
                                                router,
                                                config.id().to_string(),
                                            ))
                                        }
                                    };
                                    self.as_mut()
                                        .project_replace(Serve::Accepting { handle, config });
                                }
                                Err(e) => {
                                    break Err(satex_error!(
                                        "Serve [{}] make router error: {}",
                                        config.id(),
                                        e
                                    ))
                                }
                            }
                        }
                        _ => unreachable!(),
                    },
                    Err(e) => {
                        break Err(satex_error!(
                            "Serve [{}] create listener error: {}",
                            config.id(),
                            e
                        ))
                    }
                },

                ServeProj::Accepting { handle, config, .. } => match ready!(handle.poll(cx)) {
                    Ok(x) => break x,
                    Err(e) => {
                        break Err(satex_error!(
                            "Serve [{}] listener accept task poll error: {}",
                            config.id(),
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
            .and_then(|key| key.ok_or_else(|| satex_error!("Serve load private key error!")))
    }

    pub fn tls_boos(
        listener: TcpListener,
        router: Router,
        tls: &Tls,
        config_id: String,
    ) -> Result<Self, Error> {
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
                            info!(
                                "Serve [{}] listener accept client: {}",
                                config_id, client_addr
                            );
                            match acceptor.accept(stream).await {
                                Ok(tls_stream) => {
                                    let worker =
                                        Self::worker(TokioIo::new(tls_stream), router, client_addr);
                                    spawn(worker);
                                }
                                Err(e) => {
                                    warn!(
                                        "Serve [{}] listener accept tls stream error: {}",
                                        config_id, e
                                    );
                                }
                            };
                        }
                        Err(e) => {
                            warn!("Serve [{}] listener accept client error: {}", config_id, e);
                        }
                    }
                }
            }),
        })
    }

    pub fn boss(listener: TcpListener, router: Router, config_id: String) -> Self {
        Self {
            future: Box::pin(async move {
                loop {
                    let router = router.clone();
                    match listener.accept().await {
                        Ok((stream, client_addr)) => {
                            info!(
                                "Serve [{}] listener accept client: {}",
                                config_id, client_addr
                            );
                            let worker = Self::worker(TokioIo::new(stream), router, client_addr);
                            spawn(worker);
                        }
                        Err(e) => {
                            warn!("Serve [{}] listener accept client error: {}", config_id, e);
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

    ///
    /// 所有的Serve集合
    ///
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

///
/// 将所有的[Serve]包装成一个[SelectAll]。
///
/// 只有出现异常情况Serve才会结束，所以只要一个Serve出现异常则结束所有的任务。
///
///
impl Future for Serves {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project().serves.poll(cx) {
            Poll::Ready((Ok(_), _, _)) => Poll::Ready(Ok(())),
            Poll::Ready((Err(e), _, _)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }
}
