use std::{io::Result, net::SocketAddr};

use super::{ProxiedConnector, ProxiedStream};

struct Socks5Connector {
    stream: Box<dyn ProxiedStream>,
}

impl ProxiedConnector for Socks5Connector {
    fn extend_chain(
        connector: Box<dyn ProxiedConnector>,
        proxy_addr: SocketAddr,
        
        // This is to illustrate how to pass additional data
        password: &str,
    ) -> Result<Box<dyn ProxiedConnector>> {
        let stream = connector.connect(proxy_addr)?;
        Ok(Box::new(Socks5Connector { stream: stream }))
    }

    fn connect(self: Box<Self>, _addr: SocketAddr) -> Result<Box<dyn ProxiedStream>> {
        // do the socks5 handshake using _addr.

        Ok(self.stream)
    }
}