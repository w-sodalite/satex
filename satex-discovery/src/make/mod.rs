use satex_core::make;

use crate::{NamedServerDiscovery, ServerDiscovery};

pub mod builtin;

make! {
    MakeServerDiscovery,
    Discovery,
    (ServerDiscovery),
    NamedServerDiscovery
}

macro_rules! make_discovery {
    ($name:ident $(,)?) => {
        satex_core::make_impl!(MakeServerDiscovery,Discovery,$name);
    };
    ($name:ident,$($(#[$meta:meta])* $vis:vis $field:ident : $ty:ty),* $(,)?) => {
        satex_core::make_impl!(MakeServerDiscovery,Discovery,$name,Default,$($(#[$meta])* $vis $field : $ty),*);
    };
    ($name:ident,$mode:ident,$($(#[$meta:meta])* $vis:vis $field:ident : $ty:ty),* $(,)?) => {
        satex_core::make_impl!(MakeServerDiscovery,Discovery,$name,$mode,$($(#[$meta])* $vis $field : $ty),*);
    };
}

pub(crate) use make_discovery;
