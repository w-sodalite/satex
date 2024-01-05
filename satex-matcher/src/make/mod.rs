use satex_core::make;

use crate::{NamedRouteMatcher, RouteMatcher};

pub mod header;
pub mod host;
pub mod method;
pub mod path_regex;
pub mod path_starts_with;
pub mod query;
pub mod remote_addr;
pub mod time;

make! {
    MakeRouteMatcher,
    Matcher,
    (RouteMatcher),
    NamedRouteMatcher
}
///
/// 内部API
///
#[doc(hidden)]
#[macro_export]
macro_rules! __matcher {
    ($name:ident,$($(#[$meta:meta])* $vis:vis $field:ident : $ty:ty),* $(,)?) => {
        satex_core::make_impl!(MakeRouteMatcher,Matcher,$name,Default,$($(#[$meta])* $vis $field : $ty),*);
    };
    ($name:ident,$mode:ident,$($(#[$meta:meta])* $vis:vis $field:ident : $ty:ty),* $(,)?) => {
        satex_core::make_impl!(MakeRouteMatcher,Matcher,$name,$mode,$($(#[$meta])* $vis $field : $ty),*);
    };
}