use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use tracing::Level;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tracing {
    ///
    /// 是否以ANSI格式输出
    ///
    pub display_ansi: bool,

    ///
    /// 是否包含文件名
    ///
    pub display_file: bool,

    ///
    /// 是否显示日志目标
    ///
    pub display_target: bool,

    ///
    /// 是否显示日志级别
    ///
    pub display_level: bool,

    ///
    /// 是否显示日志行号
    ///
    pub display_line_number: bool,

    ///
    /// 是否显示线程名称
    ///
    pub display_thread_names: bool,

    ///
    /// 是否显示线程编号
    ///
    pub display_thread_ids: bool,

    ///
    /// 日志输出最大级别
    ///
    pub max_level: MaxLevel,
}

impl Default for Tracing {
    fn default() -> Self {
        Self {
            display_ansi: true,
            display_file: true,
            display_target: true,
            display_level: true,
            display_line_number: true,
            display_thread_names: true,
            display_thread_ids: true,
            max_level: Default::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MaxLevel(Level);

impl Default for MaxLevel {
    fn default() -> Self {
        Self(Level::DEBUG)
    }
}

impl From<MaxLevel> for Level {
    fn from(level: MaxLevel) -> Self {
        level.0
    }
}

impl Serialize for MaxLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MaxLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Level::from_str(&value)
            .map_err(serde::de::Error::custom)
            .map(Self)
    }
}
