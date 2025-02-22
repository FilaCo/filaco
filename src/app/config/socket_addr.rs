use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use yadi::{Container, Injectable};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FilacoSocketAddr(SocketAddr);

impl FilacoSocketAddr {
    const FILACO_SOCKET_ADDR_ENV: &'static str = "FILACO_SOCKET_ADDR";
}

impl Injectable for FilacoSocketAddr {
    fn from_container(_: &Container) -> Self {
        Self(
            env::var(Self::FILACO_SOCKET_ADDR_ENV)
                .unwrap_or_else(|_| "0.0.0.0:8080".to_string())
                .parse()
                .unwrap_or_else(|_| SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080)),
        )
    }
}

impl From<SocketAddr> for FilacoSocketAddr {
    fn from(value: SocketAddr) -> Self {
        Self(value)
    }
}

impl Into<SocketAddr> for FilacoSocketAddr {
    fn into(self) -> SocketAddr {
        self.0
    }
}
