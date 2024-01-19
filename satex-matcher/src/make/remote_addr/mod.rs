use std::net::IpAddr;

use serde::Deserialize;

pub use make::MakeRemoteAddrMatcher;
use satex_core::essential::Essential;
use satex_core::Error;

use crate::RouteMatcher;

mod make;

#[derive(Deserialize, Copy, Clone)]
pub enum Policy {
    Accept,
    Reject,
}

impl Default for Policy {
    fn default() -> Self {
        Policy::Accept
    }
}

impl Policy {
    fn as_bool(&self) -> bool {
        match self {
            Policy::Accept => true,
            Policy::Reject => false,
        }
    }
}

pub struct RemoteAddrMatcher {
    policy: Policy,
    sources: Vec<(IpAddr, u16)>,
}

impl RemoteAddrMatcher {
    pub fn new(policy: Policy, sources: Vec<(IpAddr, u16)>) -> Self {
        Self { policy, sources }
    }
}

impl RouteMatcher for RemoteAddrMatcher {
    fn is_match(&self, essential: &mut Essential) -> Result<bool, Error> {
        let source = essential.client_addr.ip();
        let policy = self.policy;
        match source {
            IpAddr::V4(source) => {
                let sn = source.octets();
                for (ip, cidr) in &self.sources {
                    match ip {
                        IpAddr::V4(ip) => {
                            let pn = ip.octets();
                            let mask: [u8; 4] = (-1_i32 << (32 - cidr)).to_be_bytes();
                            if and(sn, mask) == and(pn, mask) {
                                return Ok(policy.as_bool());
                            }
                        }
                        IpAddr::V6(_) => continue,
                    }
                }
            }
            IpAddr::V6(source) => {
                let sn = source.octets();
                for (ip, cidr) in &self.sources {
                    match ip {
                        IpAddr::V4(_) => continue,
                        IpAddr::V6(ip) => {
                            let pn = ip.octets();
                            let mask: [u8; 16] = i128::to_be_bytes(-1_i128 << (128 - cidr));
                            if and(sn, mask) == and(pn, mask) {
                                return Ok(policy.as_bool());
                            }
                        }
                    }
                }
            }
        }
        Ok(!policy.as_bool())
    }
}

fn and<const N: usize>(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
    let mut targets = [0; N];
    for index in 0..N {
        targets[index] = lhs[index] & rhs[index];
    }
    targets
}
