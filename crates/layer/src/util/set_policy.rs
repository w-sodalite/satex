use serde::Deserialize;

///
/// 设置值的策略
///
#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub enum SetPolicy {
    ///
    /// 使用新值覆盖旧值
    ///
    #[default]
    Overriding,

    ///
    /// 追加新值到旧值中
    ///
    Appending,

    ///
    /// 当不存在旧值时才设置新值
    ///
    IfNotPresent,
}
