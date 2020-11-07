#![warn(rust_2018_idioms, single_use_lifetimes)]

use std::{env, process::Command};

fn main() {
    let cfg = autocfg::new();
    cfg.emit_rustc_version(1, 31);
    cfg.emit_rustc_version(1, 33);
    cfg.emit_rustc_version(1, 39);
    cfg.emit_rustc_version(1, 46);

    if is_nightly() {
        println!("cargo:rustc-cfg=const_unstable");
    }
}

fn is_nightly() -> bool {
    env::var_os("RUSTC")
        .and_then(|rustc| Command::new(rustc).arg("--version").output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map_or(false, |version| version.contains("nightly") || version.contains("dev"))
}
