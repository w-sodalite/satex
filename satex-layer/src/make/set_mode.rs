use serde::Deserialize;

#[derive(Debug, Copy, Clone, Default, Deserialize)]
pub enum SetMode {
    Append,
    IfNotPresent,
    #[default]
    Override,
}
