use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use hyper::http::request::Parts;
use hyper::Request;

#[derive(Clone)]
pub struct Essential {
    addr: SocketAddr,
    parts: Parts,
    keep_host_header: Option<bool>,
}

impl Default for Essential {
    fn default() -> Self {
        let (parts, _) = Request::<()>::default().into_parts();
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0);
        Self::new(addr, parts)
    }
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

    ///
    /// 设置Essential拓展到当前request
    ///
    /// # Arguments
    ///
    /// * `request`: HTTP请求
    /// * `addr`: 客户端地址
    ///
    /// returns: Request<ReqBody>
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    /// use hyper::Request;
    /// use satex_core::essential::Essential;
    /// use satex_core::http::Body;
    /// let request = Request::new(Body::empty());
    /// let request = Essential::set_extension(request,SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST),3000));
    /// assert!(request.extensions().get::<Essential>().is_some());
    /// ```
    pub fn set_extension<ReqBody>(request: Request<ReqBody>, addr: SocketAddr) -> Request<ReqBody> {
        let (parts, body) = request.into_parts();
        let essential = Essential::new(addr, parts.clone());
        let mut request = Request::from_parts(parts, body);
        request.extensions_mut().insert(essential);
        request
    }
}
