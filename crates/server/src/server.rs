use crate::factory::HttpServiceFactory;
use actix_server::Server as ActixServer;
use actix_service::ServiceFactoryExt;
use actix_tls::accept::rustls_0_23::reexports::ServerConfig;
use actix_tls::accept::rustls_0_23::Acceptor as TlsAcceptor;
use http::{Request, Response};
use hyper::body::Incoming;
use hyper::service::Service as HyperService;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
use satex_core::background::BackgroundTask;
use satex_core::{BoxError, Error};
use std::fs::File;
use std::io::BufReader;
use std::net::ToSocketAddrs;

#[derive(Debug, Clone)]
pub enum Builder {
    Raw(RawBuilder),
    Tls(TlsBuilder),
}

///
/// 普通构造器
///
#[derive(Debug, Clone, Default)]
pub struct RawBuilder {
    ///
    /// 链接最大排队数量
    ///
    backlog: Option<u32>,

    ///
    /// 工作线程数量
    ///
    workers: Option<usize>,

    ///
    /// 每个工作线程允许的最大并发链接数量
    ///
    max_concurrent_connections: Option<usize>,
}

impl RawBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn workers(mut self, workers: usize) -> Self {
        self.workers = Some(workers);
        self
    }

    pub fn max_concurrent_connections(mut self, max: usize) -> Self {
        self.max_concurrent_connections = Some(max);
        self
    }

    pub fn backlog(mut self, backlog: u32) -> Self {
        self.backlog = Some(backlog);
        self
    }

    pub fn tls(self) -> TlsBuilder {
        TlsBuilder::new(self)
    }

    pub fn make_service<M>(self, make_service: M) -> Server<M> {
        Server {
            builder: Builder::Raw(self),
            make_service,
        }
    }
}

///
/// TLS构造器
///
#[derive(Debug, Clone, Default)]
pub struct TlsBuilder {
    raw: RawBuilder,
    certs: Option<String>,
    private_key: Option<String>,
    alpn_protocols: Vec<String>,
}

impl TlsBuilder {
    pub fn new(raw: RawBuilder) -> Self {
        Self {
            raw,
            certs: None,
            private_key: None,
            alpn_protocols: Default::default(),
        }
    }

    pub fn certs(mut self, cert: impl Into<String>) -> Self {
        self.certs = Some(cert.into());
        self
    }

    pub fn private_key(mut self, key: impl Into<String>) -> Self {
        self.private_key = Some(key.into());
        self
    }

    pub fn extend_alpn_protocols<I: IntoIterator<Item=P>, P: Into<String>>(
        mut self,
        protocols: I,
    ) -> Self {
        self.alpn_protocols
            .extend(protocols.into_iter().map(Into::into));
        self
    }

    pub fn alpn_protocols<I: IntoIterator<Item=P>, P: Into<String>>(
        mut self,
        protocols: I,
    ) -> Self {
        self.alpn_protocols = protocols.into_iter().map(Into::into).collect();
        self
    }

    pub fn workers(mut self, workers: usize) -> Self {
        self.raw = self.raw.workers(workers);
        self
    }

    pub fn max_concurrent_connections(mut self, max: usize) -> Self {
        self.raw = self.raw.max_concurrent_connections(max);
        self
    }

    pub fn backlog(mut self, backlog: u32) -> Self {
        self.raw = self.raw.backlog(backlog);
        self
    }

    pub fn make_service<M>(self, make_service: M) -> Server<M> {
        Server {
            builder: Builder::Tls(self),
            make_service,
        }
    }
}

///
/// 服务配置
///
pub struct Server<F> {
    builder: Builder,
    make_service: F,
}

impl Server<()> {
    pub fn builder() -> RawBuilder {
        RawBuilder::default()
    }
}

impl<M, S, ResBody> Server<M>
where
    M: HyperService<(), Response=S, Error=()> + Clone + Send + 'static,
    S: HyperService<Request<Incoming>, Response=Response<ResBody>> + Clone + 'static,
    S::Error: Into<BoxError>,
    ResBody: http_body::Body + 'static,
    ResBody::Error: Into<BoxError>,
{
    ///
    /// 服务绑定地址
    ///
    /// # Arguments
    ///
    /// * `name`: 服务名称
    /// * `server`: 服务配置
    /// * `addrs`: 绑定的地址列表
    ///
    /// returns: Result<Server, Error>
    ///
    pub fn bind<N: AsRef<str>, A: ToSocketAddrs>(
        self,
        name: N,
        addrs: A,
    ) -> Result<ActixServer, Error> {
        let Server {
            builder,
            make_service,
        } = self;
        let (config, tls_acceptor) = match builder {
            Builder::Raw(builder) => (builder, None),
            Builder::Tls(builder) => {
                let tls_acceptor = new_tls_acceptor(&builder)?;
                (builder.raw, Some(tls_acceptor))
            }
        };

        let mut builder = ActixServer::build();
        if let Some(workers) = config.workers {
            builder = builder.workers(workers);
        }
        if let Some(max_concurrent_connections) = config.max_concurrent_connections {
            builder = builder.max_concurrent_connections(max_concurrent_connections);
        }
        if let Some(backlog) = config.backlog {
            builder = builder.backlog(backlog);
        }
        builder
            .bind(name, addrs, move || match &tls_acceptor {
                Some(tls_acceptor) => actix_service::boxed::factory(
                    tls_acceptor
                        .clone()
                        .map_err(Error::new)
                        .and_then(HttpServiceFactory::new(make_service.clone())),
                ),
                None => {
                    actix_service::boxed::factory(HttpServiceFactory::new(make_service.clone()))
                }
            })
            .map(|builder| builder.run())
            .map_err(Error::new)
    }
}

///
/// 根据TLS配置创建支持HTTPS的接收器
///
/// # Arguments
///
/// * `tls`: TLS配置
///
/// returns: Result<Acceptor, Error>
///
#[inline]
fn new_tls_acceptor(tls: &TlsBuilder) -> Result<TlsAcceptor, Error> {
    let certs = load_certs(tls.certs.as_deref())?;
    let private_key = load_private_key(tls.private_key.as_deref())?;
    let mut config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, private_key)
        .map_err(Error::new)?;
    config.alpn_protocols = tls
        .alpn_protocols
        .iter()
        .map(|item| item.as_bytes().to_vec())
        .collect();
    Ok(TlsAcceptor::new(config))
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
fn load_certs(path: Option<&str>) -> Result<Vec<CertificateDer<'static>>, Error> {
    let certs = path.ok_or_else(|| Error::new("TLS is enabled, but miss `certs`!"))?;
    let file = File::open(certs).map_err(|e| Error::new(format!("Load TLS certs error: {}", e)))?;
    let mut reader = BufReader::new(file);
    rustls_pemfile::certs(&mut reader)
        .map(|x| x.map_err(Error::new))
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
fn load_private_key(path: Option<&str>) -> Result<PrivateKeyDer<'static>, Error> {
    let private_key = path.ok_or_else(|| Error::new("TLS is enabled, but miss `private_key`!"))?;
    let file = File::open(private_key).map_err(Error::new)?;
    let mut reader = BufReader::new(file);
    rustls_pemfile::private_key(&mut reader)
        .map_err(Error::new)
        .and_then(|key| {
            key.ok_or_else(|| Error::new(format!("TLS private key is invalid: {}", private_key)))
        })
}
