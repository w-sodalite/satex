use satex_core::component::Component;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Router {
    ///
    /// 全局配置
    ///
    #[serde(default)]
    pub global: Global,

    ///
    /// 路由表
    ///
    #[serde(default)]
    pub routes: Vec<Route>,
}

///
/// 路由全局配置
///
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Global {
    ///
    /// 全局Matcher集合
    ///
    #[serde(default)]
    pub matchers: Vec<Component>,

    ///
    /// 全局Layer集合
    ///
    #[serde(default)]
    pub layers: Vec<Component>,
}

///
/// 路由配置
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    ///
    /// 编号
    ///
    pub id: String,

    ///
    /// 路由Matcher集合
    ///
    #[serde(default)]
    pub matchers: Vec<Component>,

    ///
    /// 路由Layer集合
    ///
    #[serde(default)]
    pub layers: Vec<Component>,

    ///
    /// 路由Service
    ///
    pub service: Option<Component>,
}
