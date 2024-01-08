use std::net::{SocketAddr, ToSocketAddrs};
use std::str::FromStr;

use serde::{Deserialize, Deserializer};

use crate::Error;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum Endpoint {
    Ip(SocketAddr),
    Domain(String),
}

impl<'de> Deserialize<'de> for Endpoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;
        Endpoint::from_str(&text).map_err(serde::de::Error::custom)
    }
}

impl FromStr for Endpoint {
    type Err = Error;

    ///
    /// # Examples
    ///
    /// - From ip
    ///
    /// ```
    /// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    /// use std::str::FromStr;
    /// use satex_core::endpoint::Endpoint;
    /// let endpoint = Endpoint::from_str("127.0.0.1:3000").unwrap();
    /// assert_eq!(endpoint,Endpoint::Ip(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)),3000)));
    /// ```
    ///
    /// - From domain
    ///
    /// ```
    /// use std::str::FromStr;
    /// use satex_core::endpoint::Endpoint;
    /// let endpoint = Endpoint::from_str("satex.com:8080").unwrap();
    /// assert_eq!(endpoint,Endpoint::Domain(String::from("satex.com:8080")));
    /// ```
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<SocketAddr>() {
            Ok(addr) => Ok(Endpoint::Ip(addr)),
            Err(_) => Ok(Endpoint::Domain(s.to_string())),
        }
    }
}

pub enum Iter {
    Option(std::option::IntoIter<SocketAddr>),
    Vec(std::vec::IntoIter<SocketAddr>),
}

impl Iterator for Iter {
    type Item = SocketAddr;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::Option(iter) => iter.next(),
            Iter::Vec(iter) => iter.next(),
        }
    }
}

impl ToSocketAddrs for Endpoint {
    type Iter = Iter;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        match self {
            Endpoint::Ip(addr) => addr.to_socket_addrs().map(Iter::Option),
            Endpoint::Domain(domain) => domain.to_socket_addrs().map(Iter::Vec),
        }
    }
}
