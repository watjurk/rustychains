// Why this mess is necessary:
// Including rustychains_dylib in the workspace causes cargo
// to build it in a slightly different way than it would when build directly.
// This causes the interpose not to work as expect and all sorts of different trouble.

use std::{
    io::{self, Write},
    process::Command,
};

fn main() {
    println!("cargo:rerun-if-changed=./build.rs");

    // We don't want to include the 'target' folder.
    println!("cargo:rerun-if-changed=./rustychains_dylib/src");
    println!("cargo:rerun-if-changed=./rustychains_dylib/Cargo.toml");

    let output = Command::new("cargo")
        .arg("build")
        .args(["--manifest-path", "./rustychains_dylib/Cargo.toml"])
        .args(["--color", "always"])
        .output()
        .expect("Unable to build rustychains_dylib");

    if output.status.success() {
        return;
    }

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    panic!("could not compile `rustychains_dylib`");
}
