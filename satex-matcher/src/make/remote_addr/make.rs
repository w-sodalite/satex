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
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use hyper::Request;
    use satex_core::{
        config::args::{Args, Shortcut},
        essential::Essential,
        http::Body,
    };

    use crate::{MakeRouteMatcher, RouteMatcher};

    use super::MakeRemoteAddrMatcher;

    fn new_request(ip: IpAddr) -> Request<Body> {
        let request = Request::builder().body(Body::empty()).unwrap();
        let (parts, body) = request.into_parts();
        let essential = Essential::new(SocketAddr::new(ip, 80), parts.clone());
        let mut request = Request::from_parts(parts, body);
        request.extensions_mut().insert(essential);
        request
    }

    #[test]
    fn test_match() {
        let args = Args::Shortcut(Shortcut::from("127.0.0.1/24"));
        let make = MakeRemoteAddrMatcher::default();
        let matcher = make.make(args).unwrap();
        assert!(matcher
            .is_match(&new_request(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))))
            .unwrap());
        assert!(matcher
            .is_match(&new_request(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2))))
            .unwrap());
        assert!(!matcher
            .is_match(&new_request(IpAddr::V4(Ipv4Addr::new(127, 0, 2, 1))))
            .unwrap());
    }
}
