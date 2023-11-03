use libc::{c_int, sockaddr, socklen_t};

use super::interpose_macro::interpose;
use crate::INITIALIZED;

use super::darwin::{TheirSocketId, STREAMS};

interpose! {
    libc::getpeername => fn local_getpeername(fd: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int {
        if !unsafe { INITIALIZED } {
            return unsafe { libc::getpeername(fd, address, address_len) };
        }

        darwin_getpeername(fd, address, address_len)
    }

}

interpose! {
    libc::getsockname => fn local_getsockname(fd: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int {
        if !unsafe { INITIALIZED } {
            return unsafe { libc::getsockname(fd, address, address_len) };
        }

        darwin_getpeername(fd, address, address_len)
    }

}

fn darwin_getpeername(
    socket_id: TheirSocketId,
    address_ptr: *mut sockaddr,
    address_len_ptr: *mut socklen_t,
) -> c_int {
    let darwin_stream = match STREAMS.get(&socket_id) {
        None => return unsafe { libc::getpeername(socket_id, address_ptr, address_len_ptr) },
        Some(s) => s,
    };

    let metadata = &darwin_stream.1.metadata;

    let address = unsafe { &mut *address_ptr };
    *address = metadata.original_address;

    let address_len = unsafe { &mut *address_len_ptr };
    *address_len = metadata.original_address_len;
    0
}

// getsockname

// interpose! {
//     libc::select => fn local_select(nfds: c_int,
//         readfds: *mut fd_set,
//         writefds: *mut fd_set,
//         errorfds: *mut fd_set,
//         timeout: *mut timeval) -> c_int {
//         if !unsafe { INITIALIZED } {
//             return unsafe {
//                 libc::select(
//                     nfds,
//                     readfds as *mut libc::fd_set,
//                     writefds as *mut libc::fd_set,
//                     errorfds as *mut libc::fd_set,
//                     timeout,
//                 )
//             };
//         }

//         darwin_select(nfds, readfds, writefds, errorfds, timeout)
//     }
// }

// // from libc
// #[allow(non_camel_case_types)]
// struct fd_set {
//     #[cfg(all(
//         target_pointer_width = "64",
//         any(target_os = "freebsd", target_os = "dragonfly")
//     ))]
//     fds_bits: [i64; FD_SETSIZE / 64],
//     #[cfg(not(all(
//         target_pointer_width = "64",
//         any(target_os = "freebsd", target_os = "dragonfly")
//     )))]
//     fds_bits: [i32; FD_SETSIZE / 32],
// }

// fn darwin_select(
//     nfds: c_int,
//     readfds: *mut fd_set,
//     writefds: *mut fd_set,
//     errorfds: *mut fd_set,
//     timeout: *mut timeval,
// ) -> c_int {
//     let readfds_deref = unsafe { &mut *readfds };
//     let writefds_deref = unsafe { &mut *writefds };
//     let errorfds_deref = unsafe { &mut *errorfds };

//     fn rewrite_their_set(their_set: &mut fd_set) {
//         for their_id in &mut their_set.fds_bits {
//             if let Some(our_id) = THEIR_TO_OUR_SOCKET_ID.get(their_id) {
//                 *their_id = *our_id.val();
//             }
//         }
//     }

//     rewrite_their_set(readfds_deref);
//     rewrite_their_set(writefds_deref);
//     rewrite_their_set(errorfds_deref);

//     let return_value = unsafe {
//         libc::select(
//             nfds,
//             readfds as *mut libc::fd_set,
//             writefds as *mut libc::fd_set,
//             errorfds as *mut libc::fd_set,
//             timeout,
//         )
//     };

//     fn rewrite_our_set(our_set: &mut fd_set) {
//         for our_id in &mut our_set.fds_bits {
//             if let Some(their_id) = OUR_TO_THEIR_SOCKET_ID.get(our_id) {
//                 println!("{} {}", their_id.val(), our_id);
//                 *our_id = *their_id.val();
//             }
//         }
//     }

//     rewrite_our_set(readfds_deref);
//     rewrite_our_set(writefds_deref);
//     rewrite_our_set(errorfds_deref);

//     return_value
// }

// use libc::{kevent, timespec};

// interpose! {
// libc::kevent => fn new_kevent(
//     kq: c_int,
//     changelist: *const kevent,
//     nchanges: c_int,
//     eventlist: *mut kevent,
//     nevents: c_int,
//     timeout: *const timespec) -> c_int {
//     if !unsafe { INITIALIZED } {
//         return unsafe { libc::kevent(kq,changelist,nchanges,eventlist,nevents,timeout)  };
//     }

//     darwin_kevent(kq,changelist,nchanges,eventlist,nevents,timeout)
// }
// }

// fn darwin_kevent(
//     kq: c_int,
//     changelist: *const kevent,
//     nchanges: c_int,

//     eventlist: *mut kevent,
//     nevents: c_int,

//     timeout: *const timespec,
// ) -> c_int {
//     // println!("test");
//     let changes_slice = unsafe { std::slice::from_raw_parts(changelist, nchanges as usize) };
//     let mut new_changes = changes_slice.to_owned();
//     let raw_fd = 10;

//     // Rewrite changelist to use our file descriptors.
//     for change in new_changes.iter_mut() {
//         match change.filter as i16 {
//             libc::EVFILT_READ => (),
//             libc::EVFILT_WRITE => (),
//             libc::EVFILT_VNODE => (),
//             _ => continue,
//         }

//         let their_socket_id = &mut (change.ident as SocketId);

//         let our_socket_id = match STREAMS.get(their_socket_id) {
//             // Some(conn) => conn.1.as_raw_fd(),
//             Some(conn) => raw_fd,
//             None => continue,
//         };

//         change.ident = our_socket_id as usize;
//     }

//     let new_changes_ptr = new_changes.as_ptr();
//     let new_changes_len = new_changes.len() as c_int;

//     let ret_code = unsafe {
//         libc::kevent(
//             kq,
//             new_changes_ptr,
//             new_changes_len,
//             eventlist,
//             nevents,
//             timeout,
//         )
//     };

//     // Rewrite eventlist to use their file descriptors.
//     let eventlist_slice = unsafe { std::slice::from_raw_parts_mut(eventlist, nevents as usize) };
//     for event in eventlist_slice.iter_mut() {
//         match event.filter as i16 {
//             libc::EVFILT_READ => (),
//             libc::EVFILT_WRITE => (),
//             libc::EVFILT_VNODE => (),
//             _ => continue,
//         }

//         let our_socket_id = &mut (event.ident as SocketId);

//         let mut their_socket_id: Option<SocketId> = None;
//         for entry in STREAMS.iter() {
//             let l_their_socket_id = entry.0;

//             // if entry.1.as_raw_fd() == *our_socket_id {
//             if raw_fd == *our_socket_id {
//                 their_socket_id = Some(l_their_socket_id);
//                 break;
//             }
//         }

//         let their_socket_id = match their_socket_id {
//             Some(id) => id,
//             None => continue,
//         };

//         event.ident = their_socket_id as usize;
//     }

//     ret_code
// }
