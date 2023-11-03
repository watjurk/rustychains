use std::{io::Result, net::SocketAddr, sync::Mutex};

use super::{ProxiedStream, ProxiedStreamConnector};

pub struct CountBytesStreamConnector {
    stream: Option<Box<dyn ProxiedStream>>,
}

impl ProxiedStreamConnector for CountBytesStreamConnector {
    fn extend_chain(
        mut connector: Box<dyn ProxiedStreamConnector>,
        proxy_addr: SocketAddr,
    ) -> Result<Box<dyn ProxiedStreamConnector>> {
        let stream = connector.connect(proxy_addr)?;
        Ok(Box::new(CountBytesStreamConnector {
            stream: Some(stream),
        }))
    }

    fn connect(&mut self, _addr: SocketAddr) -> Result<Box<dyn ProxiedStream>> {
        Ok(CountBytesStream::new(self.stream.take().unwrap()))
    }
}

struct CountBytesStream {
    s: Box<dyn ProxiedStream>,

    bytes_read: Mutex<usize>,
    bytes_written: Mutex<usize>,
}

impl CountBytesStream {
    fn new(s: Box<dyn ProxiedStream>) -> Box<dyn ProxiedStream> {
        Box::new(CountBytesStream {
            s,
            bytes_read: Mutex::new(0),
            bytes_written: Mutex::new(0),
        })
    }
}

impl ProxiedStream for CountBytesStream {
    fn write(&self, buf: &[u8]) -> Result<usize> {
        *self.bytes_written.lock().unwrap() += buf.len();
        self.s.write(buf)
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        *self.bytes_read.lock().unwrap() += buf.len();
        self.s.read(buf)
    }
}
