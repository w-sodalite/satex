use serde::Deserialize;

#[derive(Copy, Clone, Deserialize)]
pub enum SetMode {
    Append,
    IfNotPresent,
    Override,
}

impl Default for SetMode {
    fn default() -> Self {
        SetMode::Override
    }
}
