//! An attribute for easy generation of a const function with conditional compilations.
//!
//! ## Examples
//!
//! When using like the following functions to control unstable features:
//!
//! ```toml
//! [features]
//! const_unstable = []
//! ```
//!
//! It can be written as follows:
//!
//! ```rust
//! #![cfg_attr(feature = "const_unstable", feature(const_fn))]
//! use const_fn::const_fn;
//!
//! pub struct Foo<T> {
//!     x: T,
//! }
//!
//! impl<T: Iterator> Foo<T> {
//!     /// Constructs a new `Foo`.
//!     #[const_fn(feature = "const_unstable")]
//!     pub const fn new(x: T) -> Self {
//!         Self { x }
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! Code like this will be generated:
//!
//! ```rust
//! #![cfg_attr(feature = "const_unstable", feature(const_fn))]
//!
//! pub struct Foo<T> {
//!     x: T,
//! }
//!
//! impl<T: Iterator> Foo<T> {
//!     /// Constructs a new `Foo`.
//!     #[cfg(feature = "const_unstable")]
//!     pub const fn new(x: T) -> Self {
//!         Self { x }
//!     }
//!
//!     /// Constructs a new `Foo`.
//!     #[cfg(not(feature = "const_unstable"))]
//!     pub fn new(x: T) -> Self {
//!         Self { x }
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! See [test_suite] for more examples.
//!
//! [test_suite]: https://github.com/taiki-e/const_fn/tree/master/test_suite

#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/const_fn/0.2.1")]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms, single_use_lifetimes), allow(dead_code))
))]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes, unreachable_pub)]
#![warn(clippy::all)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn_mid::ItemFn;

/// An attribute for easy generation of a const function with conditional compilations.
#[proc_macro_attribute]
pub fn const_fn(args: TokenStream, function: TokenStream) -> TokenStream {
    let args = proc_macro2::TokenStream::from(args);

    if args.is_empty() {
        return syn::Error::new_spanned(args, "`const_fn` requires an argument")
            .to_compile_error()
            .into();
    }

    let mut item: ItemFn = syn::parse_macro_input!(function);

    if item.constness.is_none() {
        return syn::Error::new_spanned(
            item.fn_token,
            "#[const_fn] attribute may only be used on const functions",
        )
        .to_compile_error()
        .into();
    }

    let mut token = quote!(#[cfg(#args)]);
    token.extend(item.to_token_stream());

    item.attrs.push(syn::parse_quote!(#[cfg(not(#args))]));
    item.constness = None;
    token.extend(item.into_token_stream());

    token.into()
}
