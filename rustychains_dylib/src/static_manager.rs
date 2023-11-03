use std::sync::{Mutex, RwLock};

use interprocess::local_socket::LocalSocketStream;
use rustychains_shared::message::{self, write_message};

pub struct StaticManager {
    unsafe_initialized: bool,
    initialized: RwLock<bool>,

    rustychains_process_stream: Mutex<Option<LocalSocketStream>>,
}

impl StaticManager {
    pub const fn new() -> Self {
        StaticManager {
            unsafe_initialized: false,
            initialized: RwLock::new(false),

            rustychains_process_stream: Mutex::new(None),
        }
    }

    pub fn is_initialized(&self) -> bool {
        // We must short-circuit this function, because in early initialization stages we would get deadlocked.
        // In short libc calls the `read` function many times during initialization, all of those calls
        // would get deadlocked wating for the mutex which itself is not initialized by libc... So there you have a deadlock. 
        if !self.unsafe_initialized {
            return false;
        }

        *self.initialized.read().unwrap()
    }

    pub fn initialize(&self, rustychains_process_stream: LocalSocketStream) {
        *self.rustychains_process_stream.lock().unwrap() = Some(rustychains_process_stream);

        *self.initialized.write().unwrap() = true;

        let borrow = &self.unsafe_initialized;
        let mut_ptr = borrow as *const bool as *mut bool;
        
        // TODO: Fix this not to be UB.
        #[allow(invalid_reference_casting)]
        unsafe { *mut_ptr = true }
    }

    pub fn write_message(&self, message: impl Into<message::M>) -> bincode::Result<()> {
        let mut option = self.rustychains_process_stream.lock().unwrap();
        let local_socket_stream = option.as_mut().unwrap();
        write_message(local_socket_stream, message)
    }
}
