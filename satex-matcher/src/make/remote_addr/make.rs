use std::net::IpAddr;

use satex_core::apply::Apply;
use satex_core::config::args::Args;
use satex_core::satex_error;
use satex_core::Error;

use crate::make::remote_addr::{Policy, RemoteAddrMatcher};
use crate::{MakeRouteMatcher, __make_matcher};

__make_matcher! {
    RemoteAddr,
    ListFlag,
    sources: Vec<String>,
    policy: Option<bool>
}

fn make(args: Args) -> Result<RemoteAddrMatcher, Error> {
    fn parse(source: &str) -> Result<(IpAddr, u16), Error> {
        match source.split_once('/') {
            None => source
                .parse::<IpAddr>()
                .map(|ip| (ip, if ip.is_ipv4() { 32 } else { 128 }))
                .map_err(|e| satex_error!(e)),
            Some((ip, cidr)) => ip
                .parse::<IpAddr>()
                .map_err(|e| satex_error!(e))
                .and_then(|ip| {
                    cidr.parse::<u16>()
                        .map(|cidr| (ip, cidr))
                        .map_err(|e| satex_error!(e))
                }),
        }
    }
    let config = Config::try_from(args)?;
    let sources = config
        .sources
        .into_iter()
        .try_fold(vec![], |sources, source| {
            parse(&source).map(|source| sources.apply(|sources| sources.push(source)))
        })?;
    Ok(RemoteAddrMatcher::new(
        config
            .policy
            .map(|flag| if flag { Policy::Accept } else { Policy::Reject })
            .unwrap_or_default(),
        sources,
    ))
}

#[cfg(test)]
mod test {
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

    use satex_core::config::args::{Args, Shortcut};

    use crate::{MakeRouteMatcher, RouteMatcher, __assert_matcher};

    use super::MakeRemoteAddrMatcher;

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::from("127.0.0.1/24"));
        __assert_matcher!(
            MakeRemoteAddrMatcher,
            args,
            [
                Ok(true) => |e| {
                    e.client_addr
                        = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3000))
                },
                Ok(true) => |e| {
                    e.client_addr
                        = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 2), 3000))
                },
                Ok(false) => |e| {
                    e.client_addr
                        = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 1, 1), 3000))
                },
            ]
        );
    }
}
