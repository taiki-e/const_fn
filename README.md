# \#\[const\_fn\]

[![Build Status](https://travis-ci.org/taiki-e/const_fn.svg?branch=master)](https://travis-ci.org/taiki-e/const_fn)
[![version](https://img.shields.io/crates/v/const_fn.svg)](https://crates.io/crates/const_fn/)
[![documentation](https://docs.rs/const_fn/badge.svg)](https://docs.rs/const_fn/)
[![license](https://img.shields.io/crates/l/const_fn.svg)](https://crates.io/crates/const_fn/)
[![Rustc Version](https://img.shields.io/badge/rustc-1.31+-lightgray.svg)](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html)

An attribute for easy generation of a const function with conditional compilations.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
const_fn = "0.2"
```

The current const_fn requires Rust 1.31 or later.

## Examples

When using like the following functions to control unstable features:

```toml
[features]
const = []
```

It can be written as follows:

```rust
#![cfg_attr(feature = "const", feature(const_fn, const_vec_new))]
use const_fn::const_fn;

#[const_fn(feature = "const")]
pub const fn empty_vec<T>() -> Vec<T> {
    Vec::new()
}
```

Code like this will be generated:

```rust
#![cfg_attr(feature = "const", feature(const_fn, const_vec_new))]

#[cfg(feature = "const")]
pub const fn empty_vec<T>() -> Vec<T> {
    Vec::new()
}

#[cfg(not(feature = "const"))]
pub fn empty_vec<T>() -> Vec<T> {
    Vec::new()
}
```

See [test_suite] for more examples.

[test_suite]: https://github.com/taiki-e/const_fn/tree/master/test_suite

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
