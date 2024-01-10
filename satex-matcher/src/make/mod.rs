use satex_core::make;

use crate::{NamedRouteMatcher, RouteMatcher};

pub mod cookie;
pub mod header;
pub mod host;
pub mod method;
pub mod path;
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
macro_rules! __make_matcher {
    ($name:ident,$($(#[$meta:meta])* $vis:vis $field:ident : $ty:ty),* $(,)?) => {
        satex_core::make_impl!(MakeRouteMatcher,Matcher,$name,Default,$($(#[$meta])* $vis $field : $ty),*);
    };
    ($name:ident,$mode:ident,$($(#[$meta:meta])* $vis:vis $field:ident : $ty:ty),* $(,)?) => {
        satex_core::make_impl!(MakeRouteMatcher,Matcher,$name,$mode,$($(#[$meta])* $vis $field : $ty),*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __assert_matcher {
    ($make:ty,$args:expr,[$($result:pat => |$essential:ident|$block:block),* $(,)?]) => {
        let make = <$make>::default();
        let matcher = make.make($args).unwrap();
        $(
            {
                let mut essential = satex_core::essential::Essential::default();
                let callback = |$essential: &mut satex_core::essential::Essential| $block;
                let _ = callback(&mut essential);
                assert!(matches!(matcher.is_match(&mut essential), $result))
            }
        )*
    };
}
