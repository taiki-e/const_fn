//! An attribute for easy generation of a const function with conditional compilations.
//!
//! # Examples
//!
//! ```rust
//! use const_fn::const_fn;
//!
//! // 1.36 and later compiler (including beta and nightly)
//! #[const_fn("1.36")]
//! pub const fn version() {
//!     /* ... */
//! }
//!
//! // nightly compiler (including dev build)
//! #[const_fn(nightly)]
//! pub const fn nightly() {
//!     /* ... */
//! }
//!
//! // `cfg(...)`
//! # #[cfg(any())]
//! #[const_fn(cfg(...))]
//! # pub fn _cfg() { unimplemented!() }
//! pub const fn cfg() {
//!     /* ... */
//! }
//!
//! // `cfg(feature = "...")`
//! #[const_fn(feature = "...")]
//! pub const fn feature() {
//!     /* ... */
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/const_fn/0.3.1")]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms, single_use_lifetimes), allow(dead_code))
))]
#![forbid(unsafe_code)]
#![warn(rust_2018_idioms, single_use_lifetimes, unreachable_pub)]
#![warn(clippy::all, clippy::default_trait_access)]
// mem::take and #[non_exhaustive] requires Rust 1.40
#![allow(clippy::mem_replace_with_default, clippy::manual_non_exhaustive)]

// older compilers require explicit `extern crate`.
#[allow(unused_extern_crates)]
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use std::str::FromStr;
use syn::{
    parse::{Parse, ParseStream},
    parse_quote, Error, *,
};
use syn_mid::ItemFn;

/// An attribute for easy generation of a const function with conditional compilations.
/// See crate level documentation for details.
#[proc_macro_attribute]
pub fn const_fn(args: TokenStream, function: TokenStream) -> TokenStream {
    let arg: Arg = syn::parse_macro_input!(args);

    let mut item: ItemFn = syn::parse_macro_input!(function);

    if item.sig.constness.is_none() {
        return Error::new_spanned(
            item.sig.fn_token,
            "#[const_fn] attribute may only be used on const functions",
        )
        .to_compile_error()
        .into();
    }

    match arg {
        Arg::Cfg(c) => {
            let mut tokens = quote!(#[cfg(#c)]);
            tokens.extend(item.to_token_stream());
            item.attrs.push(parse_quote!(#[cfg(not(#c))]));
            item.sig.constness = None;
            tokens.extend(item.into_token_stream());
            tokens.into()
        }
        Arg::Feature(f, e, s) => {
            let mut tokens = quote!(#[cfg(#f #e #s)]);
            tokens.extend(item.to_token_stream());
            item.attrs.push(parse_quote!(#[cfg(not(#f #e #s))]));
            item.sig.constness = None;
            tokens.extend(item.into_token_stream());
            tokens.into()
        }
        Arg::Version(req) => {
            if req.major > 1 || req.minor > VERSION.minor {
                item.sig.constness = None;
            }
            item.into_token_stream().into()
        }
        Arg::Nightly => {
            if !VERSION.nightly {
                item.sig.constness = None;
            }
            item.into_token_stream().into()
        }
    }
}

mod kw {
    syn::custom_keyword!(nightly);
    syn::custom_keyword!(feature);
    syn::custom_keyword!(cfg);
}

enum Arg {
    // `const_fn("1.36")`
    Version(VersionReq),
    // `const_fn(nightly)`
    Nightly,
    // `const_fn(cfg(...))`
    Cfg(TokenStream2),
    // `const_fn(feature = "...")`
    Feature(kw::feature, Token![=], LitStr),
}

impl Parse for Arg {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::nightly) {
            let _: kw::nightly = input.parse()?;
            Ok(Arg::Nightly)
        } else if lookahead.peek(kw::cfg) {
            let _: kw::cfg = input.parse()?;
            let content;
            let _: token::Paren = syn::parenthesized!(content in input);
            let t: TokenStream2 = content.parse()?;
            Ok(Arg::Cfg(t))
        } else if lookahead.peek(kw::feature) {
            let f: kw::feature = input.parse()?;
            let e: Token![=] = input.parse()?;
            let s: LitStr = input.parse()?;
            Ok(Arg::Feature(f, e, s))
        } else if lookahead.peek(LitStr) {
            let s: LitStr = input.parse()?;
            match s.value().parse::<VersionReq>() {
                Ok(req) => Ok(Arg::Version(req)),
                Err(e) => Err(Error::new(s.span(), e)),
            }
        } else {
            Err(lookahead.error())
        }
    }
}

struct VersionReq {
    major: u16,
    minor: u16,
}

impl FromStr for VersionReq {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut pieces = s.split('.');
        let major = pieces
            .next()
            .ok_or("need to specify the major version")?
            .parse::<u16>()
            .map_err(|e| e.to_string())?;
        let minor = pieces
            .next()
            .ok_or("need to specify the minor version")?
            .parse::<u16>()
            .map_err(|e| e.to_string())?;
        if let Some(s) = pieces.next() {
            Err(format!("unexpected input: {}", s))
        } else {
            Ok(Self { major, minor })
        }
    }
}

#[derive(Debug)]
struct Version {
    minor: u16,
    patch: u16,
    nightly: bool,
}

const VERSION: Version = include!(concat!(env!("OUT_DIR"), "/version.rs"));
