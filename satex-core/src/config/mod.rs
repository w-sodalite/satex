use std::collections::VecDeque;
use std::env::current_dir;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_yaml::Value;
use tracing::Level;

use crate::apply::Apply;
use crate::config::metadata::Metadata;
use crate::{satex_error, Error};

pub mod args;
pub mod metadata;

const SATEX: &str = "satex";
const SATEX_YAML: &str = "satex.yaml";

fn normalize_path(path: &str) -> Result<PathBuf, Error> {
    if path.starts_with('/') {
        Ok(PathBuf::new().apply(|buf| buf.push(path)))
    } else {
        current_dir()
            .map_err(|e| satex_error!(e))
            .map(|current| current.join(path))
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SatexConfig {
    serves: Vec<String>,

    #[serde(default)]
    tracing: Tracing,
}

impl SatexConfig {
    pub fn load(&self) -> Result<Vec<ServeConfig>, Error> {
        let mut serves = Vec::with_capacity(self.serves.len());
        for path in &self.serves {
            let path = normalize_path(path)?;
            let serve = ServeConfig::from_yaml(path)?;
            serves.push(serve);
        }
        Ok(serves)
    }

    pub fn from_yaml<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let bytes = std::fs::read(path).map_err(|e| satex_error!(e))?;
        let value =
            serde_yaml::from_slice::<Value>(bytes.as_slice()).map_err(|e| satex_error!(e))?;
        match value {
            Value::Mapping(mut value) => match value.remove(SATEX) {
                Some(value) => serde_yaml::from_value(value).map_err(|e| satex_error!(e)),
                None => Err(satex_error!("Miss field `satex`!")),
            },
            _ => Err(satex_error!("Unexpect type!")),
        }
    }

    pub fn detect() -> Result<Self, Error> {
        Self::detect_path().and_then(|path| Self::from_yaml(path))
    }
    fn detect_path() -> Result<PathBuf, Error> {
        let mut args = std::env::args()
            .map(|it| it.trim().to_string())
            .collect::<VecDeque<_>>();
        let path = loop {
            if let Some(arg) = args.pop_front() {
                if arg == "-c" || arg == "--config" {
                    if let Some(value) = args.pop_front() {
                        break Some(value);
                    }
                }
            } else {
                break None;
            }
        };
        match path {
            Some(path) => {
                if path.starts_with("/") {
                    Ok(PathBuf::new().apply(|buf| buf.push(path)))
                } else {
                    current_dir()
                        .map(|dir| dir.join(path))
                        .map_err(|e| satex_error!(e))
                }
            }
            None => current_dir()
                .map(|dir| dir.join(SATEX_YAML))
                .map_err(|e| satex_error!(e)),
        }
    }

    pub fn tracing(&self) -> &Tracing {
        &self.tracing
    }

    pub fn paths(&self) -> &[String] {
        &self.serves
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct ServeConfig {
    #[serde(default)]
    server: Server,

    #[serde(default)]
    router: Router,

    #[serde(default)]
    discovery: Vec<Metadata>,

    #[serde(default)]
    client: Client,

    #[serde(default)]
    tls: Option<Tls>,
}

impl ServeConfig {
    pub fn server(&self) -> &Server {
        &self.server
    }
    pub fn router(&self) -> &Router {
        &self.router
    }
    pub fn discovery(&self) -> &[Metadata] {
        &self.discovery
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    pub fn tls(&self) -> &Option<Tls> {
        &self.tls
    }
    pub fn from_yaml<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let bytes = std::fs::read(path).map_err(|e| satex_error!(e))?;
        serde_yaml::from_slice(bytes.as_slice()).map_err(|e| satex_error!(e))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    #[serde(default = "Server::default_port")]
    port: u16,

    #[serde(default = "Server::default_ip")]
    ip: IpAddr,

    #[serde(default)]
    tls: Option<Tls>,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            port: Server::default_port(),
            ip: Server::default_ip(),
            tls: None,
        }
    }
}

impl<'a> From<&'a Server> for SocketAddr {
    fn from(server: &'a Server) -> Self {
        SocketAddr::new(server.ip, server.port)
    }
}

impl Server {
    pub fn default_port() -> u16 {
        3000
    }

    pub fn default_ip() -> IpAddr {
        IpAddr::V4(Ipv4Addr::UNSPECIFIED)
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn ip(&self) -> IpAddr {
        self.ip
    }

    pub fn tls(&self) -> &Option<Tls> {
        &self.tls
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tracing {
    #[serde(default = "Tracing::default_max_level")]
    max_level: String,

    #[serde(default = "Tracing::enabled")]
    ansi: bool,

    #[serde(default = "Tracing::enabled")]
    level: bool,

    #[serde(default = "Tracing::enabled")]
    file: bool,

    #[serde(default = "Tracing::enabled")]
    thread_names: bool,

    #[serde(default = "Tracing::enabled")]
    line_number: bool,
}

impl Default for Tracing {
    fn default() -> Self {
        Tracing {
            max_level: Tracing::default_max_level(),
            ansi: Tracing::enabled(),
            level: Tracing::enabled(),
            file: Tracing::enabled(),
            thread_names: Tracing::enabled(),
            line_number: Tracing::enabled(),
        }
    }
}

impl Tracing {
    pub fn default_max_level() -> String {
        "info".to_string()
    }
    pub fn enabled() -> bool {
        true
    }
    pub fn max_level(&self) -> &str {
        &self.max_level
    }
    pub fn ansi(&self) -> bool {
        self.ansi
    }
    pub fn level(&self) -> bool {
        self.level
    }
    pub fn file(&self) -> bool {
        self.file
    }
    pub fn thread_names(&self) -> bool {
        self.thread_names
    }
    pub fn line_number(&self) -> bool {
        self.line_number
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Router {
    #[serde(default)]
    global: Global,

    #[serde(default)]
    trace: Trace,

    #[serde(default)]
    routes: Vec<Route>,
}

impl Router {
    pub fn routes(&self) -> &[Route] {
        self.routes.as_slice()
    }
    pub fn trace(&self) -> &Trace {
        &self.trace
    }
    pub fn global(&self) -> &Global {
        &self.global
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Global {
    #[serde(default)]
    layers: Vec<Metadata>,
}

impl Global {
    pub fn layers(&self) -> &[Metadata] {
        self.layers.as_slice()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Trace {
    #[serde(
        with = "crate::serde::tracing::level",
        default = "Trace::default_level"
    )]
    level: Level,
    #[serde(default = "Trace::default_include_headers")]
    include_headers: bool,
}

impl Default for Trace {
    fn default() -> Self {
        Self {
            level: Trace::default_level(),
            include_headers: Trace::default_include_headers(),
        }
    }
}

impl Trace {
    fn default_level() -> Level {
        Level::DEBUG
    }

    fn default_include_headers() -> bool {
        true
    }

    pub fn level(&self) -> Level {
        self.level
    }

    pub fn include_headers(&self) -> bool {
        self.include_headers
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Route {
    id: String,
    #[serde(default)]
    matchers: Vec<Metadata>,
    #[serde(default)]
    layers: Vec<Metadata>,
    service: Metadata,
}

impl Route {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn matchers(&self) -> &[Metadata] {
        &self.matchers
    }
    pub fn layers(&self) -> &[Metadata] {
        &self.layers
    }
    pub fn service(&self) -> &Metadata {
        &self.service
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Client {
    #[serde(default = "Client::default_pool_max_idle_per_host")]
    pool_max_idle_per_host: usize,

    #[serde(default = "Client::default_pool_idle_timeout_secs")]
    pool_idle_timeout_secs: u64,

    #[serde(default = "Client::default_retry_canceled_requests")]
    retry_canceled_requests: bool,

    #[serde(default = "Client::default_set_host")]
    set_host: bool,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            pool_max_idle_per_host: Self::default_pool_max_idle_per_host(),
            pool_idle_timeout_secs: Self::default_pool_idle_timeout_secs(),
            retry_canceled_requests: Self::default_retry_canceled_requests(),
            set_host: Self::default_set_host(),
        }
    }
}

impl Client {
    fn default_pool_max_idle_per_host() -> usize {
        16
    }
    fn default_pool_idle_timeout_secs() -> u64 {
        60
    }
    fn default_retry_canceled_requests() -> bool {
        true
    }
    fn default_set_host() -> bool {
        true
    }
    pub fn pool_max_idle_per_host(&self) -> usize {
        self.pool_max_idle_per_host
    }
    pub fn pool_idle_timeout_secs(&self) -> u64 {
        self.pool_idle_timeout_secs
    }
    pub fn retry_canceled_requests(&self) -> bool {
        self.retry_canceled_requests
    }
    pub fn set_host(&self) -> bool {
        self.set_host
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tls {
    certs: String,
    private_key: String,
    #[serde(default = "Tls::default_alpn_protocols")]
    alpn_protocols: Vec<String>,
}

impl Tls {
    pub fn default_alpn_protocols() -> Vec<String> {
        vec![
            "h2".to_string(),
            "http/1.1".to_string(),
            "http/1.0".to_string(),
        ]
    }

    pub fn certs(&self) -> &str {
        &self.certs
    }

    pub fn private_key(&self) -> &str {
        &self.private_key
    }

    pub fn alpn_protocols(&self) -> Vec<Vec<u8>> {
        self.alpn_protocols
            .iter()
            .map(|protocol| protocol.as_bytes().to_vec())
            .collect()
    }
}
