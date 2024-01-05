use std::net::SocketAddr;

use hyper::http::request::Parts;

#[derive(Clone)]
pub struct Essential {
    addr: SocketAddr,
    parts: Parts,
    keep_host_header: Option<bool>,
}

impl Essential {
    pub fn new(addr: SocketAddr, parts: Parts) -> Self {
        Self {
            addr,
            parts,
            keep_host_header: None,
        }
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }
    pub fn parts(&self) -> &Parts {
        &self.parts
    }
    pub fn keep_host_header(&self) -> Option<bool> {
        self.keep_host_header
    }
    pub fn set_addr(&mut self, addr: SocketAddr) {
        self.addr = addr;
    }
    pub fn set_parts(&mut self, parts: Parts) {
        self.parts = parts;
    }
    pub fn set_keep_host_header(&mut self, keep_host_header: Option<bool>) {
        self.keep_host_header = keep_host_header;
    }
}
