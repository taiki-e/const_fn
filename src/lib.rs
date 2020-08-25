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

#[macro_use]
mod utils;

mod ast;

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, TokenStream as TokenStream2, TokenTree};
use quote::{quote, ToTokens};
use std::str::FromStr;

use crate::utils::{parse_as_empty, tt_span};

/// An attribute for easy generation of a const function with conditional compilations.
/// See crate level documentation for details.
#[proc_macro_attribute]
pub fn const_fn(args: TokenStream, input: TokenStream) -> TokenStream {
    let arg = match parse_arg(args.into()) {
        Ok(a) => a,
        Err(e) => return e.into(),
    };
    let mut func = match ast::parse_input(input.into()) {
        Ok(i) => i,
        Err(e) => return e.into(),
    };

    match arg {
        Arg::Cfg(c) => {
            let mut tokens = quote!(#[cfg(#c)]);
            tokens.extend(func.to_token_stream());
            tokens.extend(quote!(#[cfg(not(#c))]));
            func.print_const = false;
            tokens.extend(func.into_token_stream());
            tokens.into()
        }
        Arg::Feature(f) => {
            let mut tokens = quote!(#[cfg(#f)]);
            tokens.extend(func.to_token_stream());
            tokens.extend(quote!(#[cfg(not(#f))]));
            func.print_const = false;
            tokens.extend(func.into_token_stream());
            tokens.into()
        }
        Arg::Version(req) => {
            if req.major > 1 || req.minor > VERSION.minor {
                func.print_const = false;
            }
            func.into_token_stream().into()
        }
        Arg::Nightly => {
            if !VERSION.nightly {
                func.print_const = false;
            }
            func.into_token_stream().into()
        }
    }
}

enum Arg {
    // `const_fn("...")`
    Version(VersionReq),
    // `const_fn(nightly)`
    Nightly,
    // `const_fn(cfg(...))`
    Cfg(TokenStream2),
    // `const_fn(feature = "...")`
    Feature(TokenStream2),
}

fn parse_arg(tokens: TokenStream2) -> Result<Arg, TokenStream2> {
    let tokens2 = tokens.clone();
    let mut iter = tokens.into_iter();

    let next = iter.next();
    match &next {
        Some(TokenTree::Ident(i)) => match i.to_string().as_str() {
            "nightly" => {
                parse_as_empty(iter)?;
                return Ok(Arg::Nightly);
            }
            "cfg" => {
                return match iter.next().as_ref() {
                    Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis => {
                        parse_as_empty(iter)?;
                        Ok(Arg::Cfg(g.stream()))
                    }
                    tt => Err(error!(tt_span(tt), "expected `(`")),
                };
            }
            "feature" => {
                return match iter.next().as_ref() {
                    Some(TokenTree::Punct(p)) if p.as_char() == '=' => match iter.next().as_ref() {
                        Some(TokenTree::Literal(l)) if l.to_string().starts_with('"') => {
                            parse_as_empty(iter)?;
                            Ok(Arg::Feature(tokens2))
                        }
                        tt => Err(error!(tt_span(tt), "expected `=`")),
                    },
                    tt => Err(error!(tt_span(tt), "expected `=`")),
                };
            }
            _ => {}
        },
        Some(TokenTree::Literal(l)) => {
            if let Ok(l) = ast::LitStr::new(l) {
                parse_as_empty(iter)?;
                return match l.value().parse::<VersionReq>() {
                    Ok(req) => Ok(Arg::Version(req)),
                    Err(e) => Err(error!(l.span(), "{}", e)),
                };
            }
        }
        _ => {}
    }

    Err(error!(
        tt_span(next.as_ref()),
        "expected one of: `nightly`, `cfg`, `feature`, string literal"
    ))
}

struct VersionReq {
    major: u16,
    minor: u16,
}

impl FromStr for VersionReq {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
