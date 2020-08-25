use proc_macro2::{Span, TokenTree};

use crate::Result;

macro_rules! error {
    ($span:expr, $msg:expr) => {{
        crate::error::Error::new($span.unwrap(), $msg)
    }};
    ($span:expr, $($tt:tt)*) => {
        error!($span, format!($($tt)*))
    };
}

pub(crate) fn tt_span(tt: Option<&TokenTree>) -> Span {
    tt.map_or_else(Span::call_site, TokenTree::span)
}

pub(crate) fn parse_as_empty(mut tokens: impl Iterator<Item = TokenTree>) -> Result<()> {
    match tokens.next() {
        Some(tt) => Err(error!(tt.span(), "unexpected token: {}", tt)),
        None => Ok(()),
    }
}
