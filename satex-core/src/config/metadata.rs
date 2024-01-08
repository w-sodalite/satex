use std::str::FromStr;

use serde::Deserialize;
use serde_yaml::Value;

use crate::config::args::{Args, Complete, Shortcut};
use crate::{satex_error, Error};

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Metadata {
    Shortcut(String),
    Complete {
        kind: String,
        #[serde(default)]
        args: Value,
    },
}

impl FromStr for Metadata {
    type Err = Error;

    fn from_str(yaml: &str) -> Result<Self, Self::Err> {
        serde_yaml::from_str(yaml).map_err(|e| satex_error!(e))
    }
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
                Some(args) => Args::Shortcut(Shortcut::from(args)),
                None => Args::Shortcut(Shortcut::none()),
            },
            Metadata::Complete { args, .. } => Args::Complete(Complete::new(args)),
        }
    }

    pub fn is_shortcut(&self) -> bool {
        matches!(self, Metadata::Shortcut(_))
    }

    pub fn is_complete(&self) -> bool {
        matches!(self, Metadata::Complete { .. })
    }
}
