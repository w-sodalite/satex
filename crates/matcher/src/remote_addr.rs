#![doc = include_str!("../docs/remote_addr.md")]

use crate::make::MakeRouteMatcher;
use crate::RouteMatcher;
use async_trait::async_trait;
use http::request::Parts;
use satex_core::component::{Args, Configurable};
use satex_core::extension::ClientAddr;
use satex_core::util::With;
use satex_core::Error;
use satex_macro::make;
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Debug, Copy, Clone, Default, Deserialize)]
pub enum Policy {
    #[default]
    Accept,
    Reject,
}

pub struct RemoteAddrRouteMatcher {
    addrs: Vec<(IpAddr, u16)>,
    policy: Policy,
}

impl RemoteAddrRouteMatcher {
    pub fn new(addrs: Vec<(IpAddr, u16)>, policy: Policy) -> Self {
        Self { addrs, policy }
    }
}

#[async_trait]
impl RouteMatcher for RemoteAddrRouteMatcher {
    async fn matches(&self, parts: &mut Parts) -> Result<bool, Error> {
        let flag = match parts.extensions.get::<ClientAddr>() {
            Some(client_addr) => in_addrs(client_addr, &self.addrs),
            None => false,
        };
        match self.policy {
            Policy::Accept => Ok(flag),
            Policy::Reject => Ok(!flag),
        }
    }
}

fn and<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
    let mut targets = [0; N];
    for index in 0..N {
        targets[index] = lhs[index] & rhs[index];
    }
    targets
}

fn in_addrs(client_addr: &ClientAddr, addrs: &[(IpAddr, u16)]) -> bool {
    match client_addr.ip() {
        IpAddr::V4(client_ip) => {
            for (addr, cidr) in addrs {
                if *cidr == 0 {
                    return true;
                }
                match addr {
                    IpAddr::V4(addr) => {
                        let mask: [u8; 4] = (-1_i32 << (32 - cidr)).to_be_bytes();
                        return and(addr.octets(), mask) == and(client_ip.octets(), mask);
                    }
                    IpAddr::V6(_) => continue,
                }
            }
        }
        IpAddr::V6(client_ip) => {
            for (addr, cidr) in addrs {
                if *cidr == 0 {
                    return true;
                }
                match addr {
                    IpAddr::V4(_) => continue,
                    IpAddr::V6(addr) => {
                        let mask: [u8; 16] = i128::to_be_bytes(-1_i128 << (128 - cidr));
                        return and(client_ip.octets(), mask) == and(addr.octets(), mask);
                    }
                }
            }
        }
    }
    false
}

#[make(kind = TailingSequence, shortcut_mode = "TailingSequence")]
struct MakeRemoteAddrRouteMatcher {
    policy: Policy,
    addrs: Vec<String>,
}

impl MakeRouteMatcher for MakeRemoteAddrRouteMatcher {
    type Matcher = RemoteAddrRouteMatcher;

    fn make(&self, args: Args) -> Result<Self::Matcher, Error> {
        let config = Config::with_args(args)?;
        let addrs = config.addrs.into_iter().try_fold(vec![], |addrs, addr| {
            match addr.split_once('/') {
                Some((addr, cidr)) => addr.parse::<IpAddr>().map_err(Error::new).and_then(|addr| {
                    cidr.parse::<u16>()
                        .map(|cidr| addrs.with(|addrs| addrs.push((addr, cidr))))
                        .map_err(Error::new)
                }),
                None => addr
                    .parse()
                    .map_err(Error::new)
                    .map(|addr| addrs.with(|addrs| addrs.push((addr, max_cidr(&addr))))),
            }
        })?;
        Ok(RemoteAddrRouteMatcher::new(addrs, config.policy))
    }
}

fn max_cidr(addr: &IpAddr) -> u16 {
    match addr {
        IpAddr::V4(_) => 32,
        IpAddr::V6(_) => 128,
    }
}
