use std::cell::RefCell;
use std::io::{Read, Result, Write};
use std::net::{SocketAddr, TcpStream};
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;

use rustychains_shared::message::Debug;

use crate::proxy_protocol::{socks4::Socks4Connector, ProxiedConnector, ProxiedStream};
use crate::proxy_protocol::{RetryError, RetryResult};
use crate::STATIC_MANAGER;

pub struct ProxyChainedStream {
    raw_tcp_stream: Arc<TcpStream>,
    stream: Box<dyn ProxiedStream>,
}

impl ProxyChainedStream {
    pub fn new(final_addr: SocketAddr) -> Result<ProxyChainedStream> {
        STATIC_MANAGER
            .write_message(Debug {
                message: format!("ProxyChainedStream new, addr: {}", final_addr),
            })
            .unwrap();

        let raw_tcp_stream_ref: Rc<RefCell<Option<Arc<TcpStream>>>> = Rc::new(RefCell::new(None));
        let tcp_connector = TcpConnector::new(raw_tcp_stream_ref.clone());

        // let proxy_addr = std::net::SocketAddr::from_str("178.48.68.61:4145").unwrap();
        // let proxied_connector = Socks4Connector::extend_chain(tcp_connector, proxy_addr)?;
        
        let proxied_connector = tcp_connector;

        let stream = proxied_connector.connect_to(final_addr)?;

        let raw_tcp_stream = raw_tcp_stream_ref
            .borrow_mut()
            .take()
            .expect("Expect to get raw_tcp_stream");

        Ok(ProxyChainedStream {
            stream,
            raw_tcp_stream,
        })
    }

    pub fn get_raw_tcp_stream(&self) -> &TcpStream {
        &self.raw_tcp_stream
    }

    pub fn write(&self, buf: &mut [u8]) -> usize {
        self.stream.write(buf).unwrap()
    }

    pub fn read(&self, buf: &mut [u8]) -> usize {
        self.stream.read(buf).unwrap()
    }

    pub fn close(&self) {
        // self.stream.close().unwrap()
    }
}

struct TcpConnector {
    tcp_stream_ref: Rc<RefCell<Option<Arc<TcpStream>>>>,
}

impl TcpConnector {
    fn new(tcp_stream_ref: Rc<RefCell<Option<Arc<TcpStream>>>>) -> Box<TcpConnector> {
        Box::new(TcpConnector { tcp_stream_ref })
    }
}

impl ProxiedConnector for TcpConnector {
    fn connect_to(self: Box<Self>, addr: SocketAddr) -> RetryResult<Box<dyn ProxiedStream>> {
        let new_stream = match ProxiedTcpStream::tcp_connect(addr) {
            Ok(s) => s,
            Err(err) => {
                // Check if the retry is possible.
                // match err.kind() {
                //     // dummy check, just to showcase the concept.
                //     std::io::ErrorKind::InvalidInput => {
                //         return RetryResult::Err(RetryError::new_no_retry(err))
                //     }
                //     _ => (),
                // }

                return RetryResult::Err(RetryError::new(self, err));
            }
        };

        *self.tcp_stream_ref.borrow_mut() = Some(new_stream.get_tcp_stream());
        Ok(Box::new(new_stream))
    }
}

struct ProxiedTcpStream {
    tcp_stream: Arc<TcpStream>,
}

impl ProxiedTcpStream {
    fn tcp_connect(addr: SocketAddr) -> Result<ProxiedTcpStream> {
        let tcp_stream = TcpStream::connect(addr)?;
        Ok(ProxiedTcpStream {
            tcp_stream: Arc::new(tcp_stream),
        })
    }

    fn get_tcp_stream(&self) -> Arc<TcpStream> {
        self.tcp_stream.clone()
    }

    unsafe fn get_tcp_stream_mut(&self) -> &mut TcpStream {
        let conn_ptr = self.tcp_stream.as_ref() as *const TcpStream as *mut TcpStream;
        
        // TODO: Fix this not to be UB.
        #[allow(invalid_reference_casting)]
        &mut *conn_ptr
    }
}

impl ProxiedStream for ProxiedTcpStream {
    fn write(&self, buf: &[u8]) -> Result<usize> {
        // SAFETY: The internal implementation of net::TcpStream `write` method in the net_imp::TcpStream struct does not require
        // self to be mutable. This constrain is unfortunately imposed by the Write trait.
        let tcp_stream_mut = unsafe { self.get_tcp_stream_mut() };
        tcp_stream_mut.write(buf)
    }

    fn read(&self, buf: &mut [u8]) -> Result<usize> {
        // SAFETY: The internal implementation of net::TcpStream `read` method in the net_imp::TcpStream struct does not require
        // self to be mutable. This constrain is unfortunately imposed by the Read trait.
        let tcp_stream_mut = unsafe { self.get_tcp_stream_mut() };
        tcp_stream_mut.read(buf)
    }

    fn close(&self) -> Result<()> {
        self.tcp_stream.shutdown(std::net::Shutdown::Both)
    }
}
