use std::{io, path::PathBuf};

use child_process::ChildProcessSpawner;
use rustychains_shared::message;

mod child_process;

fn main() {
    let dylib_path = PathBuf::from("...");
    let spawner = ChildProcessSpawner::new(dylib_path);

    let command = PathBuf::from("...");

    let mut cp = spawner
        .spawn(command, None::<[&str; 0]>)
        .expect("Unable to spawn child process");

    loop {
        match message::read_message(&mut cp.stream) {
            Ok(msg) => handle_message(msg),
            Err(err) => match *err {
                bincode::ErrorKind::Io(err) => {
                    if !matches!(err.kind(), io::ErrorKind::UnexpectedEof) {
                        panic!("{}", err)
                    }
                }

                _ => panic!("{}", err),
            },
        }
    }
}

fn handle_message(m: message::M) {
    match m {
        message::M::Error(msg) => {
            println!("{:?}", msg)
        }

        message::M::Debug(msg) => {
            println!("{:?}", msg)
        }

        _ => {
            println!("Unknown message: {:?}", m)
        }
    };
}
