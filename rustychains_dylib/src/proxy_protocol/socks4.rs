use std::{
    io::{Error, ErrorKind},
    net::{SocketAddr, SocketAddrV4},
};

use super::{ProxiedConnector, ProxiedStream, RetryError, RetryResult};

pub struct Socks4Connector {
    stream: Box<dyn ProxiedStream>,
}

impl Socks4Connector {
    pub fn extend_chain(
        connector: Box<dyn ProxiedConnector>,
        proxy_addr: SocketAddr,
    ) -> RetryResult<Box<dyn ProxiedConnector>> {
        let stream = connector.connect_to(proxy_addr)?;
        Ok(Box::new(Socks4Connector { stream }))
    }
}

impl ProxiedConnector for Socks4Connector {
    fn connect_to(self: Box<Self>, addr: SocketAddr) -> RetryResult<Box<dyn ProxiedStream>> {
        let v4_addr = match addr {
            SocketAddr::V4(a) => a,
            // TODO: make this error reporting better.
            SocketAddr::V6(_) => {
                return RetryResult::Err(RetryError::new(
                    self,
                    Error::new(
                        ErrorKind::InvalidData,
                        "Socks4Connector cannot handle ip v6 addr.",
                    ),
                ))
            }
        };

        let request_bytes = socks4_request(v4_addr);
        match self.stream.write(&request_bytes) {
            Err(e) => return RetryResult::Err(RetryError::new_no_retry(e)),
            Ok(_) => (),
        };

        let mut response_bytes: Socks4ResponseBytes = [0; 8];

        match self.stream.read(&mut response_bytes) {
            Err(e) => return RetryResult::Err(RetryError::new_no_retry(e)),
            Ok(_) => (),
        };

        let response = socks4_response(response_bytes);
        // TODO: handle response

        Ok(self.stream)
    }
}

type Socks4RequestBytes = [u8; 9];
fn socks4_request(addr: SocketAddrV4) -> Socks4RequestBytes {
    let mut request: Socks4RequestBytes = [0; 9];

    // VER
    // SOCKS version number, 0x04 for this version
    request[0] = 0x04;

    // CMD
    // 0x01 = establish a TCP/IP stream connection
    // 0x02 = establish a TCP/IP port binding
    request[1] = 0x01;

    // DSTPORT
    // 2-byte port number (in network byte order)
    let port_bytes = &mut request[2..4];
    port_bytes.copy_from_slice(&addr.port().to_be_bytes());

    // DESTIP
    // IPv4 Address, 4 bytes (in network byte order)
    let ip_bytes = &mut request[4..8];
    let ip_in_be = addr.ip().octets().map(|o| o.to_be());
    ip_bytes.copy_from_slice(&ip_in_be);

    // ID
    // the user ID string, variable length, null-terminated.
    request[8] = 0x0;

    request
}

type Socks4ResponseBytes = [u8; 8];

#[derive(Debug)]
struct Socks4Response {
    reply_code: u8,
}

fn socks4_response(response_bytes: Socks4ResponseBytes) -> Socks4Response {
    //  VN
    // reply version, null byte
    // response_bytes[0]

    // REP
    // reply code
    // Byte  Decimal	Meaning
    // 0x5A	 (90)       Request granted
    // 0x5B	 (91)       Request rejected or failed
    // 0x5C	 (92)       Request failed because client is not running identd (or not reachable from server)
    // 0x5D	 (93)       Request failed because client's identd could not confirm the user ID in the request
    let reply_code = response_bytes[1];

    Socks4Response { reply_code }
}
