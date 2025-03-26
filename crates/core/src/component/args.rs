use serde_yaml::Value;

pub enum Args<'a> {
    Full(&'a Value),
    Shortcut(Option<&'a str>),
}

impl Default for Args<'_> {
    fn default() -> Self {
        Args::Shortcut(None)
    }
}

impl<'a> Args<'a> {
    pub fn shortcut(value: &'a str) -> Self {
        Args::Shortcut(Some(value))
    }

    pub fn full(value: &'a Value) -> Self {
        Args::Full(value)
    }
}
