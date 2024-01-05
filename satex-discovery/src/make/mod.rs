use satex_core::make;

use crate::{NamedServerDiscovery, ServerDiscovery};

pub mod builtin;

make! {
    MakeServerDiscovery,
    Discovery,
    (ServerDiscovery),
    NamedServerDiscovery
}

///
/// 内部API
///
#[doc(hidden)]
#[macro_export]
macro_rules! __discovery {
    ($name:ident $(,)?) => {
        satex_core::make_impl!(MakeServerDiscovery,Discovery,$name,Default);
    };
    ($name:ident,$($(#[$meta:meta])* $vis:vis $field:ident : $ty:ty),* $(,)?) => {
        satex_core::make_impl!(MakeServerDiscovery,Discovery,$name,Default,$($(#[$meta])* $vis $field : $ty),*);
    };
    ($name:ident,$mode:ident,$($(#[$meta:meta])* $vis:vis $field:ident : $ty:ty),* $(,)?) => {
        satex_core::make_impl!(MakeServerDiscovery,Discovery,$name,$mode,$($(#[$meta])* $vis $field : $ty),*);
    };
}
