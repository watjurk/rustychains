use libc::{c_int, c_void, dlsym, size_t, sockaddr, socklen_t, ssize_t};

use lockfree::map::Map;
use once_cell::sync::Lazy;

use std::backtrace::Backtrace;
use std::net::SocketAddr;
use std::os::fd::AsRawFd;

use super::interpose_macro::interpose;

use crate::net::socket_addr::TryFromInner;
use crate::stream::ProxyChainedStream;

use crate::os::libc_error::libc_err;
use crate::STATIC_MANAGER;

pub(super) type OurSocketId = libc::c_int;
pub(super) type TheirSocketId = libc::c_int;

pub(super) struct StreamMetadata {
    pub original_address: sockaddr,
    pub original_address_len: socklen_t,
}

pub(super) struct DarwinStream {
    pub stream: ProxyChainedStream,
    pub metadata: StreamMetadata,
}

pub(super) static STREAMS: Lazy<Map<TheirSocketId, DarwinStream>> = Lazy::new(|| Map::new());

// pub(super) static OUR_TO_THEIR_SOCKET_ID: Lazy<Map<OurSocketId, TheirSocketId>> =
//     Lazy::new(|| Map::new());
// pub(super) static THEIR_TO_OUR_SOCKET_ID: Lazy<Map<TheirSocketId, OurSocketId>> =
//     Lazy::new(|| Map::new());

interpose! {
libc::connect => fn new_connect(fd: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int {
    if !STATIC_MANAGER.is_initialized()  {
        return unsafe { libc::connect(fd, address, address_len) };
    }

    darwin_connect(fd, address, address_len)
}
}

fn darwin_connect(
    their_socket_id: TheirSocketId,
    raw_address_ptr: *const sockaddr,
    address_len: socklen_t,
) -> c_int {
    // println!("darwin_connect start, sock: {}", their_socket_id);
    let raw_address = unsafe { *raw_address_ptr };
    let addr = SocketAddr::try_from_inner(raw_address).unwrap();

    let stream = ProxyChainedStream::new(addr).unwrap();
    let raw_tcp_stream = stream.get_raw_tcp_stream();

    let our_socket_id: OurSocketId = raw_tcp_stream.as_raw_fd();

    // close their file descriptor so that they cannot write to the
    // non proxied stream.
    libc_err(unsafe { libc::close(their_socket_id) }).unwrap();

    // TL;DR dup2 make so that now their_socket_id refers to our_socket_id.
    // libc docs:
    // int dup2(int oldfd, int newfd);
    // The dup2() system call performs the same task as dup(), but
    // instead of using the lowest-numbered unused file descriptor, it
    // uses the file descriptor number specified in newfd. In other
    // words, the file descriptor newfd is adjusted so that it now
    // refers to the same open file description as oldfd.
    libc_err(unsafe { libc::dup2(our_socket_id, their_socket_id) }).unwrap();

    // println!("their_socket_id: {their_socket_id}");
    // println!("our_socket_id: {our_socket_id}");

    // OUR_TO_THEIR_SOCKET_ID.insert(our_socket_id, their_socket_id);
    // THEIR_TO_OUR_SOCKET_ID.insert(their_socket_id, our_socket_id);
    let darwin_stream = DarwinStream {
        stream,
        metadata: StreamMetadata {
            original_address: raw_address,
            original_address_len: address_len,
        },
    };
    STREAMS.insert(their_socket_id, darwin_stream);

    // println!("darwin_connect end, sock: {}", their_socket_id);
    return 0;
}

interpose! {
libc::read => fn new_read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    if !STATIC_MANAGER.is_initialized() {
        return unsafe { libc::read(fd, buf, count) };
    }

    darwin_read(fd, buf, count)
}
}

fn darwin_read(socket_id: TheirSocketId, buf: *mut c_void, count: size_t) -> ssize_t {
    // println!("darwin_read start, sock: {}", socket_id);

    let darwin_stream = match STREAMS.get(&socket_id) {
        Some(s) => s,
        None => {
            let r = unsafe { libc::read(socket_id, buf, count) };
            // println!("darwin_read libc end, sock: {}", socket_id);
            return r;
        }
    };

    let slice_buf = unsafe { std::slice::from_raw_parts_mut(buf as *mut u8, count) };
    let r = darwin_stream.1.stream.read(slice_buf) as ssize_t;
    // println!("darwin_read end, sock: {}", socket_id);
    r
}

interpose! {
libc::write => fn new_write(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    if !STATIC_MANAGER.is_initialized(){
        return unsafe { libc::write(fd, buf, count) };
    }

    darwin_write(fd, buf, count)
}
}

fn darwin_write(socket_id: TheirSocketId, buf: *mut c_void, count: size_t) -> ssize_t {
    // println!("darwin_write start, sock: {}", socket_id);

    let darwin_stream = match STREAMS.get(&socket_id) {
        Some(c) => c,
        None => {
            let r = unsafe { libc::write(socket_id, buf, count) };
            // println!("darwin_write libc end, sock: {}", socket_id);
            return r;
        }
    };

    let slice_buf = unsafe { std::slice::from_raw_parts_mut(buf as *mut u8, count) };
    let r = darwin_stream.1.stream.write(slice_buf) as ssize_t;
    // println!("darwin_write end, sock: {}", socket_id);
    r
}

interpose! {
libc::close => fn new_close(fd: c_int) -> c_int {
    if !STATIC_MANAGER.is_initialized() {
        return unsafe { libc::close(fd)  };
    }

    darwin_close(fd)
}
}

fn darwin_close(socket_id: TheirSocketId) -> c_int {
    // println!("darwin_close start, sock: {}", socket_id);

    let darwin_stream = match STREAMS.remove(&socket_id) {
        Some(c) => c,
        None => {
            let r = unsafe { libc::close(socket_id) };
            // println!("darwin_close libc end, sock: {}", socket_id);
            return r;
        }
    };

    // let our_socket_id = THEIR_TO_OUR_SOCKET_ID.get(&socket_id).unwrap().1;
    // THEIR_TO_OUR_SOCKET_ID.remove(&socket_id);
    // OUR_TO_THEIR_SOCKET_ID.remove(&our_socket_id);

    // println!("darwin_close end, sock: {}", socket_id);

    darwin_stream.1.stream.close();
    0
}
