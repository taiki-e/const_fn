use std::env;
use std::process::Command;
use std::str;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if !cfg!(feature = "build_script") {
        return;
    }

    let (minor, nightly) = match rustc_minor_version() {
        Some(x) => x,
        None => return,
    };

    if minor >= 31 || nightly {
        println!("cargo:rustc-cfg=min_const_fn");
    }
    if nightly {
        println!("cargo:rustc-cfg=const_unstable");
    }
}

fn rustc_minor_version() -> Option<(u32, bool)> {
    let rustc = env::var_os("RUSTC")?;
    let output = Command::new(rustc).arg("--version").output().ok()?;
    let version = str::from_utf8(&output.stdout).ok()?;

    let nightly = version.contains("nightly");
    let mut pieces = version.split('.');
    if pieces.next() != Some("rustc 1") {
        return None;
    }
    pieces.next()?.parse().ok().map(|minor| (minor, nightly))
}