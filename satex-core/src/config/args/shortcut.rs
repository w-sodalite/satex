use std::str::Split;

use serde::de::DeserializeOwned;
use serde_yaml::{Mapping, Value};

use crate::{satex_error, Error};

pub enum GatherMode {
    Default,
    List,
    ListFlag,
    Unsupported,
}

#[derive(Clone, Copy)]
pub struct Shortcut<'a>(Option<&'a str>);

impl<'a> Shortcut<'a> {
    pub fn new(value: &'a str) -> Self {
        Self(Some(value))
    }

    pub fn none() -> Self {
        Self(None)
    }

    pub fn iter(&self) -> Iter {
        Iter(self.0.map(|v| v.split(',')))
    }

    pub fn deserialize<T: DeserializeOwned>(
        &self,
        fields: &[&'static str],
        mode: GatherMode,
    ) -> Result<T, Error> {
        let len = fields.len();
        let mut mapping = Mapping::new();
        let mut values = self.into_iter().collect::<Vec<_>>();
        match mode {
            GatherMode::Default => {
                for index in 0..len {
                    if let Some(value) = values.get(index) {
                        mapping.insert(Value::from(fields[index]), Value::from(*value));
                    }
                }
            }
            GatherMode::List => {
                if len != 1 {
                    return Err(satex_error!(
                        "Shortcut gather mode `GatherList` must have fields of size 1!"
                    ));
                }
                mapping.insert(Value::from(fields[0]), Value::from(values));
            }
            GatherMode::ListFlag => {
                if len != 2 {
                    return Err(satex_error!(
                        "Shortcut gather mode `GatherListTailFlag` must have fields of size 2!"
                    ));
                }
                if !values.is_empty() {
                    let last = values.remove(values.len() - 1);
                    if let Some(flag) = last.parse::<bool>().ok() {
                        mapping.insert(Value::from(fields[1]), Value::from(flag));
                    } else {
                        values.push(last);
                    }
                    mapping.insert(Value::from(fields[0]), Value::from(values));
                }
            }
            GatherMode::Unsupported => return Err(satex_error!("Shortcut not supported!")),
        };
        serde_yaml::from_value(Value::Mapping(mapping)).map_err(|e| satex_error!(e))
    }
}

impl<'a> IntoIterator for Shortcut<'a> {
    type Item = &'a str;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.0.map(|v| v.split(',')))
    }
}

pub struct Iter<'a>(Option<Split<'a, char>>);

impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            None => None,
            Some(split) => split.next().map(|item| item.trim()),
        }
    }
}
