use serde::Deserialize;
use serde_yaml::Value;

use crate::config::args::{Args, Complete, Shortcut};

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Metadata {
    Shortcut(String),
    Complete { kind: String, args: Value },
}

impl Metadata {
    pub fn kind(&self) -> &str {
        match self {
            Metadata::Shortcut(text) => text.split_once('=').map(|(name, _)| name).unwrap_or(text),
            Metadata::Complete { kind, .. } => kind,
        }
    }

    pub fn args(&self) -> Args {
        match self {
            Metadata::Shortcut(text) => match text.split_once('=').map(|(_, item)| item) {
                Some(args) => Args::Shortcut(Shortcut::new(args)),
                None => Args::Shortcut(Shortcut::none()),
            },
            Metadata::Complete { args, .. } => Args::Complete(Complete::new(args)),
        }
    }
}
