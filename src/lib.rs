//! An attribute for easy generation of a const function with conditional compilations.
//!
//! ## Examples
//!
//! When using like the following functions to control unstable features:
//!
//! ```toml
//! [features]
//! const = []
//! ```
//!
//! It can be written as follows:
//!
//! ```rust
//! #![cfg_attr(feature = "const", feature(const_fn, const_vec_new))]
//! use const_fn::const_fn;
//!
//! #[const_fn(feature = "const")]
//! pub const fn empty_vec<T>() -> Vec<T> {
//!     Vec::new()
//! }
//! # fn main() { let _ = empty_vec::<u8>(); }
//! ```
//!
//! Code like this will be generated:
//!
//! ```rust
//! #![cfg_attr(feature = "const", feature(const_fn, const_vec_new))]
//!
//! #[cfg(feature = "const")]
//! pub const fn empty_vec<T>() -> Vec<T> {
//!     Vec::new()
//! }
//!
//! #[cfg(not(feature = "const"))]
//! pub fn empty_vec<T>() -> Vec<T> {
//!     Vec::new()
//! }
//! # fn main() { let _ = empty_vec::<u8>(); }
//! ```
//!
//! See [test_suite] for more examples.
//!
//! [test_suite]: https://github.com/taiki-e/const_fn/tree/master/test_suite
//!

#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/const_fn/0.2.0")]
#![doc(test(attr(deny(warnings), allow(dead_code, unused_assignments, unused_variables))))]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms, unreachable_pub)]
#![warn(single_use_lifetimes)]
#![warn(clippy::all, clippy::pedantic)]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::token;
use syn_mid::ItemFn;

/// An attribute for easy generation of a const function with conditional compilations.
#[proc_macro_attribute]
pub fn const_fn(args: TokenStream, function: TokenStream) -> TokenStream {
    let args = TokenStream2::from(args);

    if args.is_empty() {
        return syn::Error::new_spanned(args, "`const_fn` requires an argument")
            .to_compile_error()
            .into();
    }

    let mut function: ItemFn = syn::parse_macro_input!(function);

    let mut const_function = function.clone();

    if function.constness.is_some() {
        function.constness = None;
    } else {
        const_function.constness = Some(token::Const::default());
    }

    function.attrs.push(syn::parse_quote!(#[cfg(not(#args))]));
    const_function.attrs.push(syn::parse_quote!(#[cfg(#args)]));

    let mut function = function.into_token_stream();
    function.extend(const_function.into_token_stream());
    TokenStream::from(function)
}
