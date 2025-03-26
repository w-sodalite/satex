mod args;
mod configurable;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

pub use args::*;
pub use configurable::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Component {
    Shortcut(String),
    Full {
        kind: String,
        #[serde(default)]
        args: Value,
    },
}

impl Component {
    pub fn kind(&self) -> &str {
        match self {
            Component::Shortcut(text) => text.split_once('=').map(|(name, _)| name).unwrap_or(text),
            Component::Full { kind, .. } => kind,
        }
    }

    pub fn args(&self) -> Args {
        match self {
            Component::Shortcut(text) => match text.split_once('=').map(|(_, item)| item) {
                Some(args) => Args::Shortcut(Some(args)),
                None => Args::Shortcut(None),
            },
            Component::Full { args, .. } => Args::Full(args),
        }
    }

    pub fn is_shortcut(&self) -> bool {
        matches!(self, Component::Shortcut(_))
    }

    pub fn is_full(&self) -> bool {
        matches!(self, Component::Full { .. })
    }
}
