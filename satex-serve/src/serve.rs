use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{ready, Context, Poll};

use bytes::Bytes;
use futures_util::future::BoxFuture;
use hyper::{Request, Response};
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use pin_project_lite::pin_project;
use tokio::net::{TcpListener, TcpStream};
use tower::Service;
use tracing::{info, warn};

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
            router: Router,
            #[pin]
            future: BoxFuture<'static, Result<TcpListener, std::io::Error>>,
        },
        Listening,
        Accepting{
            #[pin]
            handle: JoinHandle<Result<(),Error>>
        }
    }
}

impl Serve {
    pub fn new(addr: SocketAddr, router: Router) -> Self {
        Serve::Binding {
            router,
            future: Box::pin(TcpListener::bind(addr)),
        }
    }
}

impl Future for Serve {
    type Output = Result<(), Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(loop {
            match self.as_mut().project() {
                ServeProj::Binding { future, .. } => match ready!(future.poll(cx)) {
                    Ok(listener) => {
                        info!("App serve listen on: {:?}", listener);
                        match self.as_mut().project_replace(Serve::Listening) {
                            ServeProjReplace::Binding { router, .. } => {
                                let handle = spawn(EventLoop::boss(listener, router));
                                self.as_mut().project_replace(Serve::Accepting { handle });
                            }
                            _ => {
                                break Err(satex_error!(
                                    "App serve future current status `Accepting` is invalid!"
                                ));
                            }
                        }
                    }
                    Err(e) => break Err(satex_error!(e)),
                },

                ServeProj::Accepting { handle } => match ready!(handle.poll(cx)) {
                    Ok(x) => break x,
                    Err(e) => break Err(satex_error!(e)),
                },

                ServeProj::Listening => {
                    break Err(satex_error!(
                        "App serve future current status `Listening` is invalid!"
                    ));
                }
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
    pub fn boss(listener: TcpListener, router: Router) -> Self {
        Self {
            future: Box::pin(async move {
                loop {
                    let router = router.clone();
                    match listener.accept().await {
                        Ok((stream, addr)) => {
                            info!("App serve listener accept client: {}", addr);
                            let worker = Self::worker(TokioIo::new(stream), router, addr);
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

    pub fn worker(io: TokioIo<TcpStream>, router: Router, addr: SocketAddr) -> Self {
        Self {
            future: Box::pin(async move {
                Builder::new(TokioExecutor::new())
                    .serve_connection_with_upgrades(io, WorkService::new(router, addr))
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
    addr: SocketAddr,
}

impl WorkService {
    pub fn new(router: Router, addr: SocketAddr) -> Self {
        Self { router, addr }
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
        let req = Essential::set_extension(req, self.addr);
        router.call(req.map(Body::new))
    }
}
