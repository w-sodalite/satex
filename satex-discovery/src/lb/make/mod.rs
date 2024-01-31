use satex_core::make;
#[cfg(test)]
use test::new_sorted_endpoints;

use crate::lb::{LoadBalance, NamedLoadBalance};

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
macro_rules! make_load_balance {
    ($name:ident $(,)?) => {
        satex_core::make_impl!(MakeLoadBalance,LoadBalance,$name);
    };
    ($name:ident,$($(#[$meta:meta])* $vis:vis $field:ident : $ty:ty),* $(,)?) => {
        satex_core::make_impl!(MakeLoadBalance,LoadBalance,$name,Default,$($(#[$meta])* $vis $field : $ty),*);
    };
    ($name:ident,$mode:ident,$($(#[$meta:meta])* $vis:vis $field:ident : $ty:ty),* $(,)?) => {
        satex_core::make_impl!(MakeLoadBalance,LoadBalance,$name,$mode,$($(#[$meta])* $vis $field : $ty),*);
    };
}

pub(crate) use make_load_balance;

///
/// 内部API
///
macro_rules! valid_endpoints {
    ($endpoints:expr) => {{
        match $endpoints.len() {
            0 => return Ok(None),
            len => ($endpoints, len),
        }
    }};
}

pub(crate) use valid_endpoints;

#[cfg(test)]
mod test {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use satex_core::endpoint::Endpoint;

    use crate::selector::SortedEndpoint;

    const DEFAULT_PORT: u16 = 3000;

    pub fn new_sorted_endpoints(size: usize) -> Vec<SortedEndpoint> {
        (0..size)
            .into_iter()
            .map(|index| {
                SortedEndpoint::new(
                    index,
                    Endpoint::Ip(SocketAddr::new(
                        IpAddr::V4(Ipv4Addr::LOCALHOST),
                        DEFAULT_PORT + (index as u16),
                    )),
                )
            })
            .collect::<Vec<_>>()
    }
}
