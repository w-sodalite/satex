use std::fs::File;
use std::future::Future;
use std::io::BufReader;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use futures_util::future::BoxFuture;
use hyper::service::Service;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use pin_project_lite::pin_project;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpListener;
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::TlsAcceptor;
use tracing::{info, warn};

use satex_core::config::Tls;
use satex_core::essential::Essential;
use satex_core::task::spawn;
use satex_core::{satex_error, Error};

use crate::router::Router;

pin_project! {
    pub struct TcpAcceptor {
        #[pin]
        future: BoxFuture<'static, Result<() , Error>>,
    }
}

impl TcpAcceptor {
    ///
    ///
    /// 加载TLS证书文件
    ///
    /// # Arguments
    ///
    /// * `tls`: TLS配置信息
    ///
    /// returns: Result<Vec<CertificateDer, Global>, Error>
    ///
    fn load_certs(tls: &Tls) -> Result<Vec<CertificateDer<'static>>, Error> {
        let certs = tls
            .certs()
            .as_ref()
            .ok_or_else(|| satex_error!("Tls is enabled, but miss `certs`!"))?;
        let file = File::open(certs).map_err(|e| satex_error!(e))?;
        let mut reader = BufReader::new(file);
        rustls_pemfile::certs(&mut reader)
            .map(|x| x.map_err(|e| satex_error!(e)))
            .collect()
    }

    ///
    /// 加载TLS密钥文件
    ///
    /// # Arguments
    ///
    /// * `tls`: TLS配置信息
    ///
    /// returns: Result<PrivateKeyDer, Error>
    ///
    ///
    fn load_private_key(tls: &Tls) -> Result<PrivateKeyDer<'static>, Error> {
        let private_key = tls
            .private_key()
            .as_ref()
            .ok_or_else(|| satex_error!("Tls is enabled, but miss `private_key`!"))?;
        let file = File::open(private_key).map_err(|e| satex_error!(e))?;
        let mut reader = BufReader::new(file);
        rustls_pemfile::private_key(&mut reader)
            .map_err(|e| satex_error!(e))
            .and_then(|key| key.ok_or_else(|| satex_error!("Serve load private key error!")))
    }

    ///
    ///
    /// 根据TLS配置信息生成[ServerConfig]
    ///
    /// # Arguments
    ///
    /// * `tls`: TLS配置信息
    ///
    /// returns: Result<ServerConfig, Error>
    ///
    fn load_tls_config(tls: &Tls) -> Result<ServerConfig, Error> {
        let certs = Self::load_certs(tls)?;
        let private_key = Self::load_private_key(tls)?;
        let mut tls_config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, private_key)
            .map_err(|e| satex_error!(e))?;
        tls_config.alpn_protocols = tls.alpn_protocols();
        Ok(tls_config)
    }

    ///
    /// 创建一个支持TLS加密的[TcpAcceptor]
    ///
    /// # Arguments
    ///
    /// * `listener`: Tcp监听器
    /// * `router`: 路由
    /// * `tls`: TLS配置信息
    /// * `config_id`: 配置ID
    ///
    /// returns: Result<TcpAcceptor, Error>
    ///
    pub fn with_tls(
        listener: TcpListener,
        router: Router,
        tls: &Tls,
        config_id: String,
    ) -> Result<Self, Error> {
        let tls_config = Self::load_tls_config(tls)?;
        let tls_acceptor = TlsAcceptor::from(Arc::new(tls_config));
        Ok(Self {
            future: Box::pin(async move {
                loop {
                    let router = router.clone();
                    let tls_acceptor = tls_acceptor.clone();
                    match listener.accept().await {
                        Ok((stream, client_addr)) => {
                            info!(
                                "Serve [{}] listener accept client: {}",
                                config_id, client_addr
                            );
                            match tls_acceptor.accept(stream).await {
                                Ok(tls_stream) => {
                                    spawn(Self::service(
                                        TokioIo::new(tls_stream),
                                        router,
                                        client_addr,
                                    ));
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

    ///
    /// 创建一个[TcpAcceptor]，不支持TLS加密。
    ///
    /// # Arguments
    ///
    /// * `listener`: Tcp监听器
    /// * `router`: 路由
    /// * `config_id`: 配置ID
    ///
    /// returns: TcpAcceptor
    ///
    pub fn new(listener: TcpListener, router: Router, config_id: String) -> Self {
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
                            spawn(Self::service(TokioIo::new(stream), router, client_addr));
                        }
                        Err(e) => {
                            warn!("Serve [{}] listener accept client error: {}", config_id, e);
                        }
                    }
                }
            }),
        }
    }

    ///
    /// 创建HTTP请求处理服务
    ///
    /// # Arguments
    ///
    /// * `io`: IO任务执行器
    /// * `router`: 路由
    /// * `client_addr`: 客户端地址
    ///
    /// returns: Pin<Box<dyn Future<Output=Result<(), Error>>+Send, Global>>
    ///
    fn service<S>(
        io: TokioIo<S>,
        router: Router,
        client_addr: SocketAddr,
    ) -> BoxFuture<'static, Result<(), Error>>
    where
        S: AsyncRead + AsyncWrite + Send + Unpin + 'static,
    {
        Box::pin(async move {
            Builder::new(TokioExecutor::new())
                .serve_connection_with_upgrades(
                    io,
                    hyper::service::service_fn(move |request| {
                        let request = Essential::attach(request, client_addr);
                        router.call(request)
                    }),
                )
                .await
                .map_err(|e| satex_error!(e))
        })
    }
}

impl Future for TcpAcceptor {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().future.poll(cx)
    }
}
