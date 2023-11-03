use core::panic;
use std::{
    io::{self, Write},
    process::Command,
    thread, time::Duration,
};

#[test]
fn test() {
    // Start tcp server
    let go_server = thread::spawn(|| {
        let server_output = Command::new("go")
            .current_dir("./tests/go_tcp/go_tcp_server")
            .args(["run", "."])
            .output()
            .expect("Unable to run go_tcp_server");

        if !server_output.status.success() {
            io::stdout().write_all(&server_output.stdout).unwrap();
            io::stderr().write_all(&server_output.stderr).unwrap();
            panic!();
        }
    });

    thread::sleep(Duration::from_secs(2));

    // Build tcp client
    let client_build_output = Command::new("go")
        .current_dir("./tests/go_tcp/go_tcp_client")
        .args(["build", "."])
        .output()
        .expect("Unable to build go_tcp_client");

    if !client_build_output.status.success() {
        io::stdout().write_all(&client_build_output.stdout).unwrap();
        io::stderr().write_all(&client_build_output.stderr).unwrap();
        panic!();
    }

    let spawner = rustychains::child_process::ChildProcessSpawner::new(
        "./rustychains_dylib/target/debug/librustychains_dylib.dylib".into(),
    );

    let mut client = spawner
        .spawn(
            "./tests/go_tcp/go_tcp_client/go_tcp_client",
            None::<[&str; 0]>,
        )
        .expect("Unable to spawn go_tcp_client");

    let client_exit_status = client.inner.wait().expect("Unable to wait on client");
    if !client_exit_status.success() {
        io::stdout().write_all(io::read_to_string(client.inner.stdout.unwrap()).unwrap().as_bytes()).unwrap();
        io::stderr().write_all(io::read_to_string(client.inner.stderr.unwrap()).unwrap().as_bytes()).unwrap();
        panic!();
    }

    println!("{}", "go_server.join().unwrap();");

    go_server.join().unwrap();
}
