#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes)]

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    str,
};

// The rustc-cfg strings below are *not* public API. Please let us know by
// opening a GitHub issue if your build environment requires some way to enable
// these cfgs other than by executing our build script.
fn main() {
    let rustc = env::var_os("RUSTC").map_or_else(|| "rustc".into(), PathBuf::from);
    let version = match Version::from_rustc(&rustc) {
        Ok(version) => format!("{:#?}\n", version),
        Err(e) => panic!("{}", e),
    };

    let out_dir = env::var_os("OUT_DIR").map(PathBuf::from).expect("OUT_DIR not set");
    let out_file = out_dir.join("version.rs");
    fs::write(out_file, version).expect("failed to write version.rs");

    // Mark build script has been run.
    println!("cargo:rustc-cfg=const_fn_has_build_script");
}

#[derive(Debug)]
struct Version {
    minor: u32,
    nightly: bool,
}

impl Version {
    // Based on https://github.com/cuviper/autocfg/blob/1.0.1/src/version.rs#L25-L59
    //
    // Using our own parser instead of the existing crates to generate better errors.
    fn from_rustc(rustc: &Path) -> Result<Self, String> {
        let output =
            Command::new(rustc).args(&["--version", "--verbose"]).output().map_err(|e| {
                format!("failed to run `{} --version --verbose`: {}", rustc.display(), e)
            })?;
        if !output.status.success() {
            return Err("could not execute rustc".to_string());
        }
        let output = str::from_utf8(&output.stdout).map_err(|e| {
            format!("failed to parse output of `{} --version --verbose`: {}", rustc.display(), e)
        })?;

        // Find the release line in the verbose version output.
        let release = output
            .lines()
            .find(|line| line.starts_with("release: "))
            .map(|line| &line["release: ".len()..])
            .ok_or_else(|| {
                format!(
                    "could not find rustc release from output of `{} --version --verbose`: {}",
                    rustc.display(),
                    output
                )
            })?;

        // Split the version and channel info.
        let mut version_channel = release.split('-');
        let version = version_channel.next().unwrap();
        let channel = version_channel.next();

        let minor = (|| {
            // Split the version into semver components.
            let mut digits = version.splitn(3, '.');
            let major = digits.next()?;
            if major != "1" {
                return None;
            }
            let minor = digits.next()?.parse().ok()?;
            let _patch = digits.next()?;
            Some(minor)
        })()
        .ok_or_else(|| {
            format!("unexpected output from `{} --version --verbose`: {}", rustc.display(), output)
        })?;

        let nightly = channel.map_or(false, |c| c == "dev" || c == "nightly");
        Ok(Self { minor, nightly })
    }
}
