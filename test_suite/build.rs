#![warn(rust_2018_idioms, single_use_lifetimes)]

use std::{env, process::Command, str};

fn main() {
    let (minor, nightly) = match rustc_version() {
        Some(x) => x,
        None => return,
    };

    if minor >= 31 || nightly {
        println!("cargo:rustc-cfg=has_min_const_fn");
    }
    if minor >= 33 || nightly {
        println!("cargo:rustc-cfg=has_const_let");
    }
    if minor >= 39 || nightly {
        println!("cargo:rustc-cfg=has_const_vec_new");
    }
    if minor >= 46 || nightly {
        println!("cargo:rustc-cfg=has_const_match");
    }
    if nightly {
        println!("cargo:rustc-cfg=const_unstable");
    }
}

fn rustc_version() -> Option<(u32, bool)> {
    let rustc = env::var_os("RUSTC")?;
    let output = Command::new(rustc).arg("--version").output().ok()?;
    let version = str::from_utf8(&output.stdout).ok()?;
    let nightly = version.contains("nightly") || version.contains("dev");
    let mut pieces = version.split('.');
    if pieces.next() != Some("rustc 1") {
        return None;
    }
    let minor = pieces.next()?.parse().ok()?;
    Some((minor, nightly))
}
