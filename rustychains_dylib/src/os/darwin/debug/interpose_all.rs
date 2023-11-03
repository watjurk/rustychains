
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_variables)]

use std::time::SystemTime;

use libc::*;
use super::super::interpose_macro::interpose;
use crate::INITIALIZED;

interpose! {
// listen(socket: c_int, backlog: c_int) -> c_int;
libc::listen => fn local_listen(socket: c_int, backlog: c_int) -> c_int {
  if !unsafe { INITIALIZED } {
      return unsafe { libc::listen(socket, backlog) };
  }

  let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros();

  println!("{now} listen start");
  let r = unsafe { libc::listen(socket, backlog) };
  println!("{now} listen end");
  r
} 
}
interpose! {
// accept(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int;
libc::accept => fn local_accept(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int {
  if !unsafe { INITIALIZED } {
      return unsafe { libc::accept(socket, address, address_len) };
  }

  let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros();

  println!("{now} accept start");
  let r = unsafe { libc::accept(socket, address, address_len) };
  println!("{now} accept end");
  r
} 
}
interpose! {
// getsockname(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int;
libc::getsockname => fn local_getsockname(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int {
  if !unsafe { INITIALIZED } {
      return unsafe { libc::getsockname(socket, address, address_len) };
  }

  let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros();

  println!("{now} getsockname start");
  let r = unsafe { libc::getsockname(socket, address, address_len) };
  println!("{now} getsockname end");
  r
} 
}
interpose! {
// setsockopt(socket: c_int, level: c_int, name: c_int, value: *const c_void, option_len: socklen_t) -> c_int;
libc::setsockopt => fn local_setsockopt(socket: c_int, level: c_int, name: c_int, value: *const c_void, option_len: socklen_t) -> c_int {
  if !unsafe { INITIALIZED } {
      return unsafe { libc::setsockopt(socket, level, name, value, option_len) };
  }

  let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros();

  println!("{now} setsockopt start");
  let r = unsafe { libc::setsockopt(socket, level, name, value, option_len) };
  println!("{now} setsockopt end");
  r
} 
}
interpose! {
// socketpair(domain: c_int, type_: c_int, protocol: c_int, socket_vector: *mut c_int) -> c_int;
libc::socketpair => fn local_socketpair(domain: c_int, type_: c_int, protocol: c_int, socket_vector: *mut c_int) -> c_int {
  if !unsafe { INITIALIZED } {
      return unsafe { libc::socketpair(domain, type_, protocol, socket_vector) };
  }

  let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros();

  println!("{now} socketpair start");
  let r = unsafe { libc::socketpair(domain, type_, protocol, socket_vector) };
  println!("{now} socketpair end");
  r
} 
}
interpose! {
// sendto(socket: c_int, buf: *const c_void, len: size_t, flags: c_int, addr: *const sockaddr, addrlen: socklen_t) -> ssize_t;
libc::sendto => fn local_sendto(socket: c_int, buf: *const c_void, len: size_t, flags: c_int, addr: *const sockaddr, addrlen: socklen_t) -> ssize_t {
  if !unsafe { INITIALIZED } {
      return unsafe { libc::sendto(socket, buf, len, flags, addr, addrlen) };
  }

  let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros();

  println!("{now} sendto start");
  let r = unsafe { libc::sendto(socket, buf, len, flags, addr, addrlen) };
  println!("{now} sendto end");
  r
} 
}
interpose! {
// shutdown(socket: c_int, how: c_int) -> c_int;
libc::shutdown => fn local_shutdown(socket: c_int, how: c_int) -> c_int {
  if !unsafe { INITIALIZED } {
      return unsafe { libc::shutdown(socket, how) };
  }

  let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros();

  println!("{now} shutdown start");
  let r = unsafe { libc::shutdown(socket, how) };
  println!("{now} shutdown end");
  r
} 
}
