//! Types and traits used within lexer callbacks.
//!
//! When lexing, it's very common to want to do *something* whenever you encounter a certain token - you might want to
//! convert a string slice into an integer for a hypothetical `Number` token, or perhaps want to process escape
//! sequences for a hypothetical `StringLiteral` token. Logos allows you to perform this kind of processing using
//! *callbacks*, which are attached to token variants and are called by the lexer when a token matches.
//!
//! Callbacks take one argument (a mutable reference to a lexer) and can return any value implementing the
//! [CallbackResult] trait. This trait is implemented for a few types already, and their behaviour is listed below.
//!
//! For **unit variants** (variants like `Token::Variant`, which don't contain data):
//!
//! | Type            | Effect                                                                                         |
//! |-----------------|------------------------------------------------------------------------------------------------|
//! | `bool`          | If `true`, emits `Token::Unit`. Otherwise, emits a generic "unknown token" error.              |
//! | `Skip`          | Skips the matched token                                                                        |
//! | `Filter<()>`    | If `Filter::Accept(())`, emits `Token::Unit`. Otherwise, skips the matched token.              |
//! | `Option<()>`    | If `Some(())`, emits `Token::Unit`. Otherwise, emits a generic "unknown token" error.          |
//! | `Result<(), E>` | If `Ok(())`, emits `Token::Unit`. If `Err(E)`, emits the contained error value.                |
//!
//! For **value variants** (variants like `Token::Value(C)`, which contain one piece of data):
//!
//! | Type           | Effect                                                                                           |
//! |----------------|--------------------------------------------------------------------------------------------------|
//! | `Filter<C>`    | If `Filter::Accept(C)`, creates and emits `Token::Value(C)`. Otherwise, skips the matched token. |
//! | `Option<()>`   | If `Some(C)`, emits `Token::Value(C)`. Otherwise, emits a generic "unknown token" error.         |
//! | `Result<C, E>` | If `Ok(C)`, creates and emits `Token::Value(C)`. If `Err(E)`, emits the contained error value.   |
//!
//! The [Output] type can also be returned from callbacks, and how it behaves depends on the data inside:
//! - If `Output::Skip`, skips the matched token.
//! - If `Output::Construct(())`, emits `Token::Unit`.
//! - If `Output::Construct(C)`, creates and emits `Token::Value(C)`.
//! - If `Output::Emit(T)`, emits the token `T` as-is.
//! - If `Output::Error(E)`, emits the error value.
//!
//! In order to support more sophisticated lexing strategies, Logos also allows callbacks to manipulate the lexer
//! itself. The [Lexer] documentation contains more details, but you're most likely interested in [Lexer::remainder] and
//! [Lexer::bump].

use crate::{Filter, Lexer, Logos, Skip};

/// Represents actions the lexer can take.
///
/// This type is used as part of the [CallbackResult] trait, and may also be returned from callbacks. After executing a
/// callback, the lexer will use the `Output` value to determine what to do next.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Output<C, T, E> {
    /// Skip this token
    Skip,
    /// Construct a variant containing a value of type `C`
    Construct(C),
    /// Emit a token of type `T`
    Emit(T),
    /// Emit an error of type `E`
    Error(E),
}

/// Types that can be returned from lexer callbacks.
///
/// This trait is implemented for many types already, and there probably isn't much reason to implement it yourself. See
/// the table in the [module-level documentation](./index.html) for an overview.
pub trait CallbackResult<'s, C, T>
where
    T: Logos<'s>,
{
    /// Construct an [Output] value using `self`, instructing the lexer how to proceed. See `Output`'s documentation for
    /// more information.
    fn construct(self, lex: &Lexer<'s, T>) -> Output<C, T, T::Error>;
}

impl<'s, C, T> CallbackResult<'s, C, T> for C
where
    T: Logos<'s>,
{
    #[inline]
    fn construct(self, _lex: &Lexer<'s, T>) -> Output<C, T, T::Error> {
        Output::Construct(self)
    }
}

impl<'s, T> CallbackResult<'s, (), T> for bool
where
    T: Logos<'s>,
{
    #[inline]
    fn construct(self, lex: &Lexer<'s, T>) -> Output<(), T, T::Error> {
        match self {
            true => Output::Construct(()),
            false => Output::Error(Lexer::error(lex)),
        }
    }
}

impl<'s, C, T> CallbackResult<'s, C, T> for Output<C, T, T::Error>
where
    T: Logos<'s>,
{
    #[inline(always)]
    fn construct(self, _lex: &Lexer<'s, T>) -> Output<C, T, T::Error> {
        self
    }
}

impl<'s, C, T> CallbackResult<'s, C, T> for Option<C>
where
    T: Logos<'s>,
{
    #[inline]
    fn construct(self, lex: &Lexer<'s, T>) -> Output<C, T, T::Error> {
        match self {
            Some(contents) => Output::Construct(contents),
            None => Output::Error(Lexer::error(lex)),
        }
    }
}

impl<'s, C, T, E> CallbackResult<'s, C, T> for Result<C, E>
where
    T: Logos<'s>,
    E: Into<T::Error>,
{
    #[inline]
    fn construct(self, _lex: &Lexer<'s, T>) -> Output<C, T, T::Error> {
        match self {
            Ok(contents) => Output::Construct(contents),
            Err(error) => Output::Error(error.into()),
        }
    }
}

impl<'s, T> CallbackResult<'s, (), T> for Skip
where
    T: Logos<'s>,
{
    #[inline]
    fn construct(self, _lex: &Lexer<'s, T>) -> Output<(), T, T::Error> {
        Output::Skip
    }
}

impl<'s, C, T> CallbackResult<'s, C, T> for Filter<C>
where
    T: Logos<'s>,
{
    #[inline]
    fn construct(self, _lex: &Lexer<'s, T>) -> Output<C, T, T::Error> {
        match self {
            Filter::Accept(contents) => Output::Construct(contents),
            Filter::Skip => Output::Skip,
        }
    }
}
