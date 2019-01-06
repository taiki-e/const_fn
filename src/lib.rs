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
//! #![cfg_attr(feature = "const", feature(const_fn, const_let, const_vec_new))]
//! # #[macro_use]
//! # extern crate const_fn;
//!
//! #[const_fn(feature = "const")]
//! pub const fn empty_vec<T>() -> Vec<T> {
//!     let vec = Vec::new();
//!     vec
//! }
//! # fn main() { let _ = empty_vec::<u8>(); }
//! ```
//!
//! Code like this will be generated:
//!
//! ```rust
//! #![cfg_attr(feature = "const", feature(const_fn, const_let, const_vec_new))]
//!
//! #[cfg(feature = "const")]
//! pub const fn empty_vec<T>() -> Vec<T> {
//!     let vec = Vec::new();
//!     vec
//! }
//!
//! #[cfg(not(feature = "const"))]
//! pub fn empty_vec<T>() -> Vec<T> {
//!     let vec = Vec::new();
//!     vec
//! }
//! # fn main() { let _ = empty_vec::<u8>(); }
//! ```
//!
//! ## Rust Version
//!
//! The current minimum required Rust version is 1.30.
//!

#![crate_type = "proc-macro"]
#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/const_fn/0.1.2")]

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_quote, ItemFn};

/// An attribute for easy generation of a const function with conditional compilations.
#[proc_macro_attribute]
pub fn const_fn(args: TokenStream, function: TokenStream) -> TokenStream {
    #[inline(never)]
    fn compile_err(msg: &str) -> TokenStream {
        TokenStream::from(quote!(compile_error!(#msg);))
    }

    if args.is_empty() {
        return compile_err("`const_fn` requires an argument");
    }

    let mut function: ItemFn = match syn::parse(function) {
        Err(_) => return compile_err("`const_fn` may only be used on functions"),
        Ok(f) => f,
    };

    let mut const_function = function.clone();

    match &function.constness {
        Some(_) => function.constness = None,
        None => const_function.constness = Some(Default::default()),
    }

    let args = TokenStream2::from(args);
    function.attrs.push(parse_quote!(#[cfg(not(#args))]));
    const_function.attrs.push(parse_quote!(#[cfg(#args)]));

    let mut function = function.into_token_stream();
    function.extend(const_function.into_token_stream());
    TokenStream::from(function)
}
