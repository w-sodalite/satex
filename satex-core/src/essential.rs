use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use hyper::http::Extensions;
use hyper::{HeaderMap, Method, Request, Uri, Version};

use crate::apply::Apply;

#[derive(Debug, Clone)]
pub struct PathVariables(pub HashMap<String, String>);

pub struct Display {
    pub client_addr: SocketAddr,
    pub method: Method,
    pub uri: Uri,
    pub version: Version,
    pub route_id: Option<String>,
}

impl Debug for Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Essential")
            .field("version", &self.version)
            .field("method", &self.method)
            .field("uri", &self.uri)
            .field("client_addr", &self.client_addr)
            .field("route_id", &self.route_id)
            .finish()
    }
}

#[derive(Clone)]
#[non_exhaustive]
pub struct Essential {
    pub client_addr: SocketAddr,
    pub method: Method,
    pub uri: Uri,
    pub version: Version,
    pub headers: HeaderMap,
    pub extensions: Extensions,
    pub route_id: Option<String>,
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
            route_id: None,
        };
        Request::from_parts(parts, body).apply(|request| request.extensions_mut().insert(essential))
    }

    pub fn display(&self) -> Display {
        Display {
            version: self.version.clone(),
            method: self.method.clone(),
            uri: self.uri.clone(),
            client_addr: self.client_addr,
            route_id: self.route_id.clone(),
        }
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
            route_id: None,
        }
    }
}
