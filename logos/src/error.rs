//! Types and traits for error handling in Logos.
//!
//! Unfortunately, the world is not sunshine and rainbows, so - at some point - your lexer will receive invalid input.
//! What "invalid" looks like for you depends on circumstance, but regardless of what you're doing it's important to be
//! able to accurately report that *something went wrong*.
//!
//! As part of that reporting process, you might also want to include some details about what happened, like the
//! specific error's cause, or where you found the error. All of this is information that can be used later down the
//! track to help somebody (maybe even you!) figure out what went wrong.
//!
//! By default, Logos uses the [UnknownToken] type to represent the standard fare lexing error: an unknown character or
//! sequence of characters was found. For most people, this information is perfectly sufficient for their purposes, and
//! anything else would be overkill. However, sometimes that information *isn't* enough, and you'd like to convey a more
//! specific cause or provide more information along with the error. To that end, Logos allows you to use your own error
//! type when lexing, and additionally allows you to control *how* an error is constructed when a lexer encounters an error.
//!
//! See [Error]'s documentation if you'd like to implement it for your own type. Otherwise, you may be interested in the [Logos]
//! trait's documentation, which covers how to use a type implementing `Error` with Logos.

use crate::{Lexer, Logos};
use std::fmt::{Display, Formatter};

/// A trait for representing errors that occur during lexing.
///
/// In most cases, this is just an unknown token, so Logos provides [UnknownToken] and will use that as the error type
/// by default. Because of this, it's likely that you'll never actually *need* to implement this trait yourself.
///
/// However, some more advanced use-cases (such as whitespace-sensitive lexers for languages like Python) warrant the
/// ability to emit more detailed error information from within a lexer callback. In those scenarios, you can use this
/// trait combined with the derive macro's `error` option to customise how errors are constructed. See the [Logos]
/// trait's documentation for more information.
pub trait Error<'source, T>
where
    T: Logos<'source>,
{
    /// Creates an error value representing an unknown token. This error value may optionally use context from the
    /// lexer to provide more useful diagnostics.
    fn unknown_token(lex: &Lexer<'source, T>) -> Self;
}

/// The primary error case when lexing, and the default error type in Logos.
///
/// This type carries no extra information of its own (not even span information!) and is used simply to indicate that
/// an error occurred.
///
/// When deriving [Logos], this is used as the default error type. See the trait's documentation for more information.
#[derive(Debug, Default, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct UnknownToken;

impl<'source, T> Error<'source, T> for UnknownToken
where
    T: Logos<'source>,
{
    #[inline(always)]
    fn unknown_token(_lex: &Lexer<'source, T>) -> Self {
        UnknownToken
    }
}

impl Display for UnknownToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown token encountered while lexing")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for UnknownToken {}
