use std::fs::File;
use std::future::Future;
use std::io::BufReader;
use std::net::SocketAddr;
use std::pin::{pin, Pin};
use std::sync::Arc;
use std::task::{Context, Poll};

use async_stream::try_stream;
use futures::future::{select_all, BoxFuture, SelectAll};
use futures::{Stream, StreamExt};
use hyper::service::Service;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use pin_project_lite::pin_project;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::TlsAcceptor;
use tracing::{info, warn};

use satex_core::config::{ServeConfig, Tls};
use satex_core::essential::Essential;
use satex_core::{satex_error, Error};

use crate::router::Router;

pin_project! {
    pub struct Serve {
        #[pin]
        future: BoxFuture<'static, Result<(), Error>>,
    }
}

impl Future for Serve {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project().future.poll(cx)
    }
}

///
/// 创建服务监听任务
///
/// # Arguments
///
/// * `config`: 配置信息
///
/// returns: Serve
///
#[inline]
pub fn serve(config: ServeConfig) -> Serve {
    Serve {
        future: Box::pin(start(config)),
    }
}

///
/// 启动服务监听任务
///
/// # Arguments
///
/// * `config`: 配置信息
///
/// returns: Result<(), Error>
///
#[inline]
async fn start(config: ServeConfig) -> Result<(), Error> {
    let router = Router::try_from(&config)?;
    let tls_acceptor = make_tls_acceptor(&config)?;
    let listener = TcpListener::bind(config.server().bind_addr())
        .await
        .map_err(|e| satex_error!(e))?;
    info!("Serve [{}] start success!", config.id());
    let mut stream = pin!(accept_stream(listener));
    while let Some(accepted) = stream.next().await {
        let router = router.clone();
        let tls_acceptor = tls_acceptor.clone();
        match accepted {
            Ok((stream, client_addr)) => {
                info!(
                    "Serve [{}] listener accept client: {}",
                    config.id(),
                    client_addr
                );
                match tls_acceptor {
                    Some(tls_acceptor) => {
                        match tls_acceptor.accept(stream).await {
                            Ok(stream) => spawn_stream(stream, router, client_addr),
                            Err(e) => {
                                warn!(
                                    "Serve [{}] listener accept tls stream error: {}",
                                    config.id(),
                                    e
                                );
                            }
                        };
                    }
                    None => spawn_stream(stream, router, client_addr),
                };
            }
            Err(e) => {
                warn!(
                    "Serve [{}] listener accept client error: {}",
                    config.id(),
                    e
                );
            }
        }
    }
    Ok(())
}

///
/// 绑定并创建一个Tcp Stream流
///
/// # Arguments
///
/// * `bind_addr`: 绑定地址
///
/// returns: impl Stream<Item=Result<(TcpStream, SocketAddr), Error>>+Sized
///
#[inline]
fn accept_stream(
    listener: TcpListener,
) -> impl Stream<Item = Result<(TcpStream, SocketAddr), Error>> {
    try_stream! {
        loop {
            let (stream,client_addr) = listener.accept().await.map_err(|e|satex_error!(e))?;
            yield (stream,client_addr);
        }
    }
}

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
#[inline]
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
#[inline]
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
/// 根据TLS配置信息生成[TlsAcceptor]
///
/// # Arguments
///
/// * `tls`: TLS配置信息
///
/// returns: Result<ServerConfig, Error>
///
#[inline]
fn make_tls_acceptor(config: &ServeConfig) -> Result<Option<TlsAcceptor>, Error> {
    let tls = config.server().tls();
    if tls.enabled() {
        let certs = load_certs(tls)?;
        let private_key = load_private_key(tls)?;
        let mut tls_config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, private_key)
            .map_err(|e| satex_error!(e))?;
        tls_config.alpn_protocols = tls.alpn_protocols();
        Ok(Some(TlsAcceptor::from(Arc::new(tls_config))))
    } else {
        Ok(None)
    }
}

///
/// 执行客户端请求
///
/// # Arguments
///
/// * `io`: IO任务执行器
/// * `router`: 路由
/// * `client_addr`: 客户端地址
///
/// returns: Pin<Box<dyn Future<Output=Result<(), Error>>+Send, Global>>
///
#[inline]
fn spawn_stream<S>(stream: S, router: Router, client_addr: SocketAddr)
where
    S: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    spawn(async move {
        if let Err(e) = Builder::new(TokioExecutor::new())
            .serve_connection_with_upgrades(
                TokioIo::new(stream),
                hyper::service::service_fn(move |request| {
                    let request = Essential::attach(request, client_addr);
                    router.call(request)
                }),
            )
            .await
        {
            warn!("Spawn client stream error: {}", e);
        }
    });
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

///
/// 将所有的[Serve]包装成一个[SelectAll]。
///
/// 只有出现异常情况Serve才会结束，所以只要一个Serve出现异常则结束所有的任务。
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

impl FromIterator<Serve> for Serves {
    fn from_iter<T: IntoIterator<Item = Serve>>(iter: T) -> Self {
        let serves = select_all(iter);
        Self { serves }
    }
}
