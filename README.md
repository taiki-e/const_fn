# \#\[const\_fn\]

[![crates-badge]][crates-url]
[![docs-badge]][docs-url]
[![license-badge]][license]
[![rustc-badge]][rustc-url]

[crates-badge]: https://img.shields.io/crates/v/const_fn.svg
[crates-url]: https://crates.io/crates/const_fn
[docs-badge]: https://docs.rs/const_fn/badge.svg
[docs-url]: https://docs.rs/const_fn
[license-badge]: https://img.shields.io/crates/l/const_fn.svg
[license]: #license
[rustc-badge]: https://img.shields.io/badge/rustc-1.31+-lightgray.svg
[rustc-url]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html

An attribute for easy generation of a const function with conditional compilations.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
const_fn = "0.3"
```

The current const_fn requires Rust 1.31 or later.

## Examples

When using like the following functions to control unstable features:

```toml
[features]
const_unstable = []
```

It can be written as follows:

```rust
#![cfg_attr(feature = "const_unstable", feature(const_fn))]
use const_fn::const_fn;

pub struct Foo<T> {
    x:T,
}

impl<T: Iterator> Foo<T> {
    /// Constructs a new `Foo`.
    #[const_fn(feature = "const_unstable")]
    pub const fn new(x: T) -> Self {
        Self { x }
    }
}
```

Code like this will be generated:

```rust
#![cfg_attr(feature = "const_unstable", feature(const_fn))]

pub struct Foo<T> {
    x:T,
}

impl<T: Iterator> Foo<T> {
    /// Constructs a new `Foo`.
    #[cfg(feature = "const_unstable")]
    pub const fn new(x: T) -> Self {
        Self { x }
    }

    /// Constructs a new `Foo`.
    #[cfg(not(feature = "const_unstable"))]
    pub fn new(x: T) -> Self {
        Self { x }
    }
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
