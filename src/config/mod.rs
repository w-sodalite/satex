use crate::config::router::Router;
use crate::config::server::Server;
use crate::config::tracing::Tracing;
use satex_core::component::Component;
use satex_core::Error;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub mod router;
pub mod server;
pub mod tracing;

///
/// 配置文件
///
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Config {
    ///
    /// 服务配置
    ///
    #[serde(default)]
    pub server: Server,

    ///
    /// 路由配置
    ///
    #[serde(default)]
    pub router: Router,

    ///
    /// 服务发现组件
    ///
    pub discoveries: Vec<Component>,

    ///
    /// 日志配置
    ///
    #[serde(default)]
    pub tracing: Tracing,
}

impl Config {
    pub fn from_yaml<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let bytes = std::fs::read(path).map_err(Error::new)?;
        serde_yaml::from_slice(&bytes).map_err(Error::new)
    }
}
