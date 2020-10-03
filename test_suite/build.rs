#![warn(rust_2018_idioms, single_use_lifetimes)]

use std::{env, process::Command, str};

fn main() {
    let (minor, nightly) = match rustc_version() {
        Some(x) => x,
        None => return,
    };

    if minor >= 31 {
        println!("cargo:rustc-cfg=has_min_const_fn");
    }
    if minor >= 33 {
        println!("cargo:rustc-cfg=has_const_let");
    }
    if minor >= 39 {
        println!("cargo:rustc-cfg=has_const_vec_new");
    }
    if minor >= 46 {
        println!("cargo:rustc-cfg=has_const_match");
    }
    if nightly {
        println!("cargo:rustc-cfg=const_unstable");
    }
}

fn rustc_version() -> Option<(u32, bool)> {
    let rustc = env::var_os("RUSTC")?;
    let output = Command::new(rustc).args(&["--version", "--verbose"]).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let output = str::from_utf8(&output.stdout).ok()?;

    // Find the release line in the verbose version output.
    let release = output
        .lines()
        .find(|line| line.starts_with("release: "))
        .map(|line| &line["release: ".len()..])?;

    // Split the version and channel info.
    let mut version_channel = release.split('-');
    let version = version_channel.next().unwrap();
    let channel = version_channel.next();

    // Split the version into semver components.
    let mut digits = version.splitn(3, '.');
    let major = digits.next()?;
    if major != "1" {
        return None;
    }
    let minor = digits.next()?.parse().ok()?;
    let _patch = digits.next()?;

    let nightly = channel.map_or(false, |c| c == "dev" || c == "nightly");
    Some((minor, nightly))
}
