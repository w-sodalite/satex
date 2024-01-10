use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use satex_core::endpoint::Endpoint;
use satex_core::make;

use crate::lb::{LoadBalance, NamedLoadBalance};
use crate::selector::IndexedEndpoint;

pub mod ip_hash;
pub mod random;
pub mod sequential;
pub mod standby;
pub mod weight;

make! {
    MakeLoadBalance,
    LoadBalance,
    (LoadBalance),
    NamedLoadBalance
}

///
/// 内部API
///
#[doc(hidden)]
#[macro_export]
macro_rules! __make_load_balance {
    ($name:ident $(,)?) => {
        satex_core::make_impl!(MakeLoadBalance,LoadBalance,$name,Default);
    };
    ($name:ident,$($(#[$meta:meta])* $vis:vis $field:ident : $ty:ty),* $(,)?) => {
        satex_core::make_impl!(MakeLoadBalance,LoadBalance,$name,Default,$($(#[$meta])* $vis $field : $ty),*);
    };
    ($name:ident,$mode:ident,$($(#[$meta:meta])* $vis:vis $field:ident : $ty:ty),* $(,)?) => {
        satex_core::make_impl!(MakeLoadBalance,LoadBalance,$name,$mode,$($(#[$meta])* $vis $field : $ty),*);
    };
}

///
/// 内部API
///
#[doc(hidden)]
#[macro_export]
macro_rules! valid_endpoints {
    ($endpoints:expr) => {{
        match $endpoints.len() {
            0 => return Ok(None),
            len => ($endpoints, len),
        }
    }};
}

#[cfg(test)]
fn new_endpoints(port: u16, size: usize) -> Vec<IndexedEndpoint> {
    (0..size)
        .into_iter()
        .map(|index| {
            IndexedEndpoint::new(
                index,
                Endpoint::Ip(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::LOCALHOST),
                    port + (index as u16),
                )),
            )
        })
        .collect::<Vec<_>>()
}
