use std::future::Future;
use std::pin::Pin;
use std::task::{ready, Context, Poll};

use futures::future::{select_all, BoxFuture, SelectAll};
use pin_project_lite::pin_project;
use tokio::net::TcpListener;
use tokio::spawn;
use tokio::task::JoinHandle;
use tracing::info;

use satex_core::config::ServeConfig;
use satex_core::{satex_error, Error};

use crate::acceptor::TcpAcceptor;
use crate::router::make::MakeRouter;

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
                                    let tls = config.server().tls();
                                    let handle = if tls.enabled() {
                                        info!("Serve [{}] create listener success!", config.id());
                                        match TcpAcceptor::with_tls(
                                            listener,
                                            router,
                                            tls,
                                            config.id().to_string(),
                                        ) {
                                            Ok(acceptor) => spawn(acceptor),
                                            Err(e) => break Err(satex_error!(e)),
                                        }
                                    } else {
                                        info!("Serve [{}] create listener success!", config.id());
                                        spawn(TcpAcceptor::new(
                                            listener,
                                            router,
                                            config.id().to_string(),
                                        ))
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
