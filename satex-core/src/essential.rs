use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use hyper::http::Extensions;
use hyper::{HeaderMap, Method, Request, Uri, Version};

use crate::apply::Apply;

#[derive(Debug, Clone)]
pub struct PathVariables(pub HashMap<String, String>);

#[derive(Clone)]
#[non_exhaustive]
pub struct Essential {
    pub client_addr: SocketAddr,
    pub method: Method,
    pub uri: Uri,
    pub version: Version,
    pub headers: HeaderMap,
    pub extensions: Extensions,
}

impl Essential {
    pub fn attach<ReqBody>(request: Request<ReqBody>, client_addr: SocketAddr) -> Request<ReqBody> {
        let (parts, body) = request.into_parts();
        let essential = Essential {
            client_addr,
            method: parts.method.clone(),
            uri: parts.uri.clone(),
            version: parts.version,
            headers: parts.headers.clone(),
            extensions: Extensions::default(),
        };
        Request::from_parts(parts, body).apply(|request| request.extensions_mut().insert(essential))
    }
}

impl Default for Essential {
    fn default() -> Self {
        Self {
            client_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)),
            method: Method::default(),
            uri: Uri::default(),
            version: Version::default(),
            headers: HeaderMap::default(),
            extensions: Extensions::default(),
        }
    }
}
