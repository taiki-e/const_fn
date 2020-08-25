use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rustc-cfg=const_fn_has_build_script");

    let version = match Version::new() {
        Some(version) => format!("{:#?}\n", version),
        None => panic!("unexpected output from `rustc --version`"),
    };

    let out_dir = env::var_os("OUT_DIR").map(PathBuf::from).expect("OUT_DIR not set");
    let out_file = out_dir.join("version.rs");
    fs::write(out_file, version).expect("failed to write version.rs");
}

#[derive(Debug)]
struct Version {
    minor: u16,
    patch: u16,
    nightly: bool,
}

impl Version {
    fn new() -> Option<Self> {
        let (version, channel, _date) = version_check::triple()?;
        let (_major, minor, patch) = version.to_mmp();
        let nightly = channel.is_nightly() || channel.is_dev();
        Some(Version { minor, patch, nightly })
    }
}
