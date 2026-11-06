use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use ip_discovery::blocking::get_ipv4;
use local_ip_address::local_ip;

pub struct Hosts {
    port: u16,
}

impl Hosts {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub fn get_external(&self) -> SocketAddr {
        match get_ipv4() {
            Ok(provider) => SocketAddr::new(provider.ip, self.port),
            Err(_) => self.get_broadcast(),
        }
    }

    pub fn get_internal(&self) -> SocketAddr {
        match local_ip() {
            Ok(ip) => SocketAddr::new(ip, self.port),
            Err(_) => self.get_broadcast(),
        }
    }

    pub fn get_broadcast(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), self.port)
    }
}
