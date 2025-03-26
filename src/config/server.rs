use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

const DEFAULT_SERVER_PORT: u16 = 3000;

///
/// 服务配置
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    ///
    /// 监听端口
    ///
    #[serde(default = "default_port")]
    pub port: u16,

    ///
    /// 监听地址
    ///
    #[serde(default = "default_host")]
    pub host: IpAddr,

    ///
    /// TLS配置
    ///
    #[serde(default)]
    pub tls: Tls,

    ///
    /// 工作线程
    ///
    pub workers: Option<usize>,

    ///
    /// 每个工作线程允许的并发连接数
    ///
    pub max_concurrent_connections: Option<usize>,

    ///
    /// 每个工作线程允许的最大排队数量
    ///
    pub backlog: Option<u32>,
}

fn default_port() -> u16 {
    DEFAULT_SERVER_PORT
}

fn default_host() -> IpAddr {
    IpAddr::V4(Ipv4Addr::UNSPECIFIED)
}

impl Default for Server {
    fn default() -> Self {
        Self {
            port: default_port(),
            host: default_host(),
            tls: Tls::default(),
            workers: None,
            max_concurrent_connections: None,
            backlog: None,
        }
    }
}

///
/// TLS配置
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tls {
    ///
    /// 是否开启TLS
    ///
    pub enabled: bool,

    ///
    /// 证书路径
    ///
    pub certs: Option<String>,

    ///
    /// 私钥路径
    ///
    pub private_key: Option<String>,

    ///
    /// 支持的ALPN协议
    ///
    pub alpn_protocols: Vec<String>,
}

impl Default for Tls {
    fn default() -> Self {
        Self {
            enabled: false,
            certs: None,
            private_key: None,
            alpn_protocols: vec![
                "h2".to_string(),
                "http/1.1".to_string(),
                "http/1.0".to_string(),
            ],
        }
    }
}
