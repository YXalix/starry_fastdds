//! [ArceOS](https://github.com/rcore-os/arceos) network module.
//!
//! It provides unified networking primitives for TCP/UDP communication
//! using various underlying network stacks. Currently, only [smoltcp] is
//! supported.
//!
//! # Organization
//!
//! - [`TcpSocket`]: A TCP socket that provides POSIX-like APIs.
//! - [`UdpSocket`]: A UDP socket that provides POSIX-like APIs.
//! - [`dns_query`]: Function for DNS query.
//!
//! # Cargo Features
//!
//! - `smoltcp`: Use [smoltcp] as the underlying network stack. This is enabled
//!   by default.
//!
//! [smoltcp]: https://github.com/smoltcp-rs/smoltcp

#![no_std]
#![feature(ip_in_core)]
#![feature(new_uninit)]

#[macro_use]
extern crate log;
extern crate alloc;

cfg_if::cfg_if! {
    if #[cfg(feature = "smoltcp")] {
        mod smoltcp_impl;
        use smoltcp_impl as net_impl;
    }
}

mod netlink_impl;

pub use self::netlink_impl::NetlinkSocket;
pub use self::net_impl::TcpSocket;
pub use self::net_impl::UdpSocket;
pub use self::net_impl::{bench_receive, bench_transmit};
pub use self::net_impl::{dns_query, from_core_sockaddr, into_core_sockaddr, poll_interfaces};
pub use smoltcp::time::Duration;
pub use smoltcp::wire::{IpAddress as IpAddr, IpEndpoint, Ipv4Address as Ipv4Addr};

use axdriver::{prelude::*, AxDeviceContainer};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct SocketAddr {
    pub addr: IpAddr,
    pub port: u16,
}

impl Default for SocketAddr {
    fn default() -> Self {
        SocketAddr {
            addr: IpAddr::v4(0, 0, 0, 0),
            port: 0,
       }
    }
}

impl SocketAddr {
    pub fn new(addr: IpAddr, port: u16) -> Self {
        SocketAddr { addr, port }
    }

    pub fn new_netlink(_groups: u32) -> Self {
        SocketAddr {
            ..Default::default()
        }
    }
}

impl Into<IpEndpoint> for SocketAddr {
    fn into(self) -> IpEndpoint {
        IpEndpoint {
            addr: self.addr,
            port: self.port,
        }
    }
}

impl From<IpEndpoint> for SocketAddr {
    fn from(ie: IpEndpoint) -> Self {
        SocketAddr {
            addr: ie.addr,
            port: ie.port,
        }
    }
}

/// Initializes the network subsystem by NIC devices.
pub fn init_network(mut net_devs: AxDeviceContainer<AxNetDevice>) {
    info!("Initialize network subsystem...");

    let dev = net_devs.take_one().expect("No NIC device found!");
    info!("  use NIC 0: {:?}", dev.device_name());
    net_impl::init(dev);
    netlink_impl::init();
}
