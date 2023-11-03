use std::ffi::OsStr;
use std::io;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

use interprocess::local_socket::LocalSocketStream;
use rustychains_shared::{config, os};

pub struct ChildProcessSpawner {
    dylib_path: PathBuf,
}

impl ChildProcessSpawner {
    pub fn new(dylib_path: PathBuf) -> ChildProcessSpawner {
        ChildProcessSpawner {
            dylib_path: dylib_path,
        }
    }

    pub fn spawn<COMMAND, ARGS, ARG>(
        &self,
        command: COMMAND,
        args: Option<ARGS>,
    ) -> io::Result<ChildProcess>
    where
        COMMAND: AsRef<OsStr>,
        ARGS: IntoIterator<Item = ARG>,
        ARG: AsRef<OsStr>,
    {
        let mut child_command = Command::new(command);
        if let Some(args) = args {
            child_command.args(args);
        }

        child_command.stdout(Stdio::piped());
        child_command.stderr(Stdio::piped());

        child_command.env(os::DYLD_INSERT_LIBRARIES_ENV_NAME, &self.dylib_path);
        child_command.env("RUST_BACKTRACE", "1");

        let socket_address = new_interprocess_socket_address();
        child_command.env(config::SOCKET_ADDRESS_ENV_VAR, socket_address.clone());

        let child_process = child_command.spawn()?;
        let listener =
            interprocess::local_socket::LocalSocketListener::bind(socket_address.clone())?;
        let stream = listener.accept()?;

        Ok(ChildProcess {
            inner: child_process,
            stream: stream,
        })
    }
}

pub struct ChildProcess {
    pub inner: Child,
    pub stream: LocalSocketStream,
}

impl ChildProcess {}

const SOCKET_NAME: &str = "_rustychains.sock";
fn new_interprocess_socket_address() -> String {
    let current_time_string = &chrono::Utc::now().timestamp_millis().to_string();

    let socket_addr: String = match interprocess::local_socket::NameTypeSupport::query() {
        interprocess::local_socket::NameTypeSupport::OnlyPaths => {
            let mut addr = std::env::temp_dir();
            addr.push(String::from(current_time_string) + SOCKET_NAME);
            addr.into_os_string().into_string().unwrap()
        }

        interprocess::local_socket::NameTypeSupport::OnlyNamespaced
        | interprocess::local_socket::NameTypeSupport::Both => {
            let mut addr = String::from("@");
            addr += current_time_string;
            addr += SOCKET_NAME;
            addr
        }
    };

    return socket_addr;
}
