pub use complete::Complete;
pub use shortcut::{GatherMode, Shortcut};

mod complete;
mod shortcut;

#[derive(Clone)]
pub enum Args<'a> {
    Shortcut(Shortcut<'a>),
    Complete(Complete<'a>),
}

impl<'a> From<Complete<'a>> for Args<'a> {
    fn from(value: Complete<'a>) -> Self {
        Args::Complete(value)
    }
}

impl<'a> From<Shortcut<'a>> for Args<'a> {
    fn from(value: Shortcut<'a>) -> Self {
        Args::Shortcut(value)
    }
}

impl<'a> Default for Args<'a> {
    fn default() -> Self {
        Args::Shortcut(Shortcut::none())
    }
}

#[macro_export]
macro_rules! config {
    ($comp:ident, $($(#[$meta:meta])* $vis:vis $field:ident : $ty:ty),* $(,)?) => {
        $crate::config!($comp,Default,$($(#[$meta])* $vis $field:$ty),*);
    };
    ($comp:ident, $mode:ident, $($(#[$meta:meta])* $vis:vis $field:ident : $ty:ty),* $(,)?) => {
        #[derive(serde::Deserialize)]
        struct Config {
            $(
                $(#[$meta])*
                $vis $field : $ty
            ),*
        }

        impl TryFrom<satex_core::config::args::Args<'_>> for Config {
            type Error = $crate::Error;

            fn try_from(args: Args) -> Result<Self,$crate::Error> {
                match args {
                    satex_core::config::args::Args::Shortcut(shortcut) => shortcut.deserialize(&[$(stringify!($field)),*],satex_core::config::args::GatherMode::$mode),
                    satex_core::config::args::Args::Complete(complete) => complete.deserialize(),
                }.map_err(|e|$crate::satex_error!("`{}` deserialize args error: {}", stringify!($comp) , e))
            }
        }
    };
}
