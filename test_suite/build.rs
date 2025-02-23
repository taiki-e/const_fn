// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{env, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!(
        "cargo:rustc-check-cfg=cfg(rustc_1_31,rustc_1_33,rustc_1_39,rustc_1_46,rustc_1_61,rustc_1_83,const_unstable)"
    );

    let cfg = autocfg::new();
    if cfg.probe_rustc_version(1, 31) {
        println!("cargo:rustc-cfg=rustc_1_31");
    }
    if cfg.probe_rustc_version(1, 33) {
        println!("cargo:rustc-cfg=rustc_1_33");
    }
    if cfg.probe_rustc_version(1, 39) {
        println!("cargo:rustc-cfg=rustc_1_39");
    }
    if cfg.probe_rustc_version(1, 46) {
        println!("cargo:rustc-cfg=rustc_1_46");
    }
    if cfg.probe_rustc_version(1, 61) {
        println!("cargo:rustc-cfg=rustc_1_61");
    }
    if cfg.probe_rustc_version(1, 83) {
        println!("cargo:rustc-cfg=rustc_1_83");
    }

    if is_nightly() {
        println!("cargo:rustc-cfg=const_unstable");
    }
}

fn is_nightly() -> bool {
    env::var_os("RUSTC")
        .and_then(|rustc| Command::new(rustc).arg("--version").output().ok())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .is_some_and(|version| version.contains("nightly") || version.contains("dev"))
}
