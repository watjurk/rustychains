use std::{io::Result, net::SocketAddr};

pub mod socks4;

pub type RetryResult<T> = std::result::Result<T, RetryError>;

pub struct RetryError {
    pub retry_connector: Option<Box<dyn ProxiedConnector>>,
    pub error: std::io::Error,
}

impl RetryError {
    pub fn new(retry_connector: Box<dyn ProxiedConnector>, error: std::io::Error) -> RetryError {
        RetryError {
            retry_connector: Some(retry_connector),
            error,
        }
    }

    pub fn new_no_retry(error: std::io::Error) -> RetryError {
        RetryError {
            retry_connector: None,
            error,
        }
    }
}

impl From<RetryError> for std::io::Error {
    fn from(value: RetryError) -> std::io::Error {
        value.error
    }
}


impl std::fmt::Debug for RetryError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

pub trait ProxiedConnector {
    fn connect_to(self: Box<Self>, addr: SocketAddr) -> RetryResult<Box<dyn ProxiedStream>>;
}

pub trait ProxiedStream: Send + Sync {
    fn read(&self, buf: &mut [u8]) -> Result<usize>;
    fn write(&self, buf: &[u8]) -> Result<usize>;
    fn close(&self) -> Result<()>;
}
