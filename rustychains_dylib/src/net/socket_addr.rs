use libc::sockaddr;
use std::backtrace::Backtrace;
use std::error::Error;

use std::mem;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

pub trait TryFromInner<Inner>: Sized {
    fn try_from_inner(value: Inner) -> Result<Self, Box<dyn Error>>;
}

impl TryFromInner<sockaddr> for SocketAddr {
    fn try_from_inner(value: sockaddr) -> Result<Self, Box<dyn Error>> {
        match value.sa_family as libc::c_int {
            libc::AF_INET => {
                SocketAddrV4::try_from_inner(value).and_then(|x| Ok(SocketAddr::V4(x)))
            }

            libc::AF_INET6 => {
                SocketAddrV6::try_from_inner(value).and_then(|x| Ok(SocketAddr::V6(x)))
            }

            _ => Err("Unknown Socket family".into()),
        }
    }
}

impl TryFromInner<sockaddr> for SocketAddrV4 {
    fn try_from_inner(raw_addr: sockaddr) -> Result<Self, Box<dyn Error>> {
        let port_bytes: [u8; 2] = [raw_addr.sa_data[1], raw_addr.sa_data[0]].map(|x| x as u8);
        let port = u16::from_ne_bytes(port_bytes);

        let ip_bytes: [u8; 4] = [
            raw_addr.sa_data[2],
            raw_addr.sa_data[3],
            raw_addr.sa_data[4],
            raw_addr.sa_data[5],
        ]
        .map(|x| x as u8);
        let ip = Ipv4Addr::new(ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]);

        Ok(SocketAddrV4::new(ip, port))
    }
}

impl TryFromInner<sockaddr> for SocketAddrV6 {
    fn try_from_inner(_value: sockaddr) -> Result<Self, Box<dyn Error>> {
        todo!("impl TryFromInner<sockaddr> for SocketAddrV6")
    }
}
