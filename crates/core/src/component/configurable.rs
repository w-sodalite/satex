use crate::component::Args;
use crate::Error;
use serde::de::DeserializeOwned;
use serde_yaml::Value;
use std::cmp::min;

const COMMA: char = ',';

const COLON: char = ':';

const LF: char = '\n';

const LINE: char = '-';

const SPACE: char = ' ';

pub trait Configurable: DeserializeOwned {
    ///
    /// 配置字段
    ///
    const FIELDS: &'static [&'static str];

    ///
    /// 配置类关联的Make名称
    ///
    const COMPANION: &'static str;

    ///
    /// 配置快捷模式
    ///
    const SHORTCUT_MODE: ShortcutMode;

    ///
    /// 使用Args反序列化自身
    ///
    fn with_args(args: Args) -> Result<Self, Error> {
        match args {
            Args::Full(value) => serde_yaml::from_value((*value).clone()).map_err(Error::new),
            Args::Shortcut(None) => serde_yaml::from_value(Value::default()).map_err(Error::new),
            Args::Shortcut(Some(value)) => {
                let mut builder = String::default();
                let fields = Self::FIELDS;
                let shortcut_mode = Self::SHORTCUT_MODE;
                let companion = Self::COMPANION;
                match shortcut_mode {
                    ShortcutMode::Object => {
                        let values = value.split(COMMA).map(|value| value.trim());
                        let size = min(fields.len(), values.clone().count());
                        fields
                            .iter()
                            .take(size)
                            .zip(values.take(size))
                            .for_each(|(k, v)| {
                                append_k_v(&mut builder, k, v);
                            });
                    }
                    ShortcutMode::Sequence => {
                        if fields.len() != 1 {
                            return Err(Error::new(format!(
                                "[{}] configure with shortcut mode `Sequence` must have exactly 1 field",
                                companion
                            )));
                        }
                        let field = fields[0];
                        append_k_seq(&mut builder, field, value);
                    }
                    ShortcutMode::TailingSequence => {
                        if fields.len() != 2 {
                            return Err(Error::new(format!(
                                "[{}] configure with shortcut mode [TailingSequence] must have exactly 2 fields",
                                companion
                            )));
                        }
                        let k1 = fields[0];
                        let k2 = fields[1];
                        match value.rsplit_once(COMMA) {
                            Some((v1, v2)) => {
                                append_k_v(&mut builder, k1, v1);
                                append_k_seq(&mut builder, k2, v2);
                            }
                            None => {
                                return Err(Error::new(format!(
                                    "[{}] configure with shortcut mode [TailingSequence] value must contains `,`",
                                    companion
                                )));
                            }
                        }
                    }
                    ShortcutMode::Unsupported => {
                        return Err(Error::new(format!(
                            "[{}] configure unsupported shortcut",
                            companion
                        )));
                    }
                };
                serde_yaml::from_str(&builder).map_err(Error::new)
            }
        }
    }
}

///
/// 快捷模式 - 参数模式
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShortcutMode {
    Object,
    Sequence,
    TailingSequence,
    Unsupported,
}

fn append_k_v(builder: &mut String, key: &str, value: &str) {
    builder.push_str(key);
    builder.push(COLON);
    builder.push(SPACE);
    builder.push_str(value);
    builder.push(LF);
}

fn append_k_seq(builder: &mut String, key: &str, value: &str) {
    builder.push_str(key);
    builder.push(COLON);
    builder.push(LF);
    value.split(COMMA).for_each(|item| {
        builder.push(LINE);
        builder.push(SPACE);
        builder.push_str(item.trim());
        builder.push(LF);
    });
}
