use crate::error::Error;
use crate::iter::MapWithLexer;
use crate::source::Source;
use crate::Logos;

use core::fmt::{self, Debug};
use core::mem::ManuallyDrop;

/// A byte range in the source.
pub type Span = core::ops::Range<usize>;

// This is basically a slightly less ugly way of writing `fn(Result<Token, Token::Error>, Lexer<'source, Token>) ->
// Result<(Token, Span), Token::Error>`. It also looks like a cute little type-level function.
//
// Being forced to disambiguate associated types truly is of the Devil.
type ErrorOf<'s, T> = <T as Logos<'s>>::Error;
type ResultOf<'s, T, U> = Result<U, ErrorOf<'s, T>>;
type SpanFn<'s, T> = fn(ResultOf<'s, T, T>, &Lexer<'s, T>) -> ResultOf<'s, T, (T, Span)>;

/// A `Lexer` allows you to read through a source (a type implementing the [Source] trait, like a string
/// slice) and produce tokens using the [Logos] trait. It's important to note that you should *not* implement [Logos]
/// yourself, and should always use the derive macro instead. See the [trait's documentation][Logos] for more details.
///
/// # Example
/// ```
/// use logos::{Logos, UnknownToken};
///
/// #[derive(Logos, Debug, PartialEq)]
/// enum Example {
///     #[regex(r"[ \n\t\f]+", logos::skip)]
///     Whitespace,
///
///     #[regex("-?[0-9]+", |lex| lex.slice().parse().ok())]
///     Integer(i64),
///
///     #[regex(r"-?[0-9]+\.[0-9]+", |lex| lex.slice().parse().ok())]
///     Float(f64),
/// }
///
/// let tokens: Vec<_> = Example::lexer("108 9 3.5 4.2 a").collect();
///
/// assert_eq! {
///     tokens,
///     &[
///         Ok(Example::Integer(108)),
///         Ok(Example::Integer(9)),
///         Ok(Example::Float(3.5)),
///         Ok(Example::Float(4.2)),
///         Err(UnknownToken), // We don't recognise this token!
///     ]
/// }
/// ```
pub struct Lexer<'source, Token: Logos<'source>> {
    pub(crate) source: &'source Token::Source,
    pub(crate) token: ManuallyDrop<Option<Result<Token, Token::Error>>>,
    pub(crate) token_start: usize,
    pub(crate) token_end: usize,

    /// The "extras" associated with `Token`.
    pub extras: Token::Extras,
}

impl<'source, Token> Debug for Lexer<'source, Token>
where
    Token: Logos<'source>,
    Token::Source: Debug,
    Token::Extras: Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_map()
            .entry(&"source", &self.source)
            .entry(&"extras", &self.extras)
            .finish()
    }
}

impl<'source, Token: Logos<'source>> Lexer<'source, Token> {
    /// Create a new `Lexer`.
    ///
    /// Because of type inference, it can be more convenient to use the [Logos::lexer] function from the [Logos] trait
    /// instead.
    pub fn new(source: &'source Token::Source) -> Self
    where
        Token::Extras: Default,
    {
        Self::with_extras(source, Default::default())
    }

    /// Create a new `Lexer` with the provided extras.
    ///
    /// Because of type inference, it can be more convenient to use the [Logos::lexer_with_extras] function from the
    /// [Logos] trait instead.
    ///
    /// # Note
    ///
    /// In most cases, you can use [Lexer::new] instead. You should only use this function if you need to set up your
    /// lexer in a way that doesn't play nicely with the [Default] trait.
    pub fn with_extras(source: &'source Token::Source, extras: Token::Extras) -> Self {
        Lexer {
            source,
            token: ManuallyDrop::new(None),
            extras,
            token_start: 0,
            token_end: 0,
        }
    }

    /// Wrap the lexer in an [Iterator] that pairs tokens with their source positions.
    ///
    /// The iterator produces `Result<(Token, Span), Token::Error>` values.
    ///
    /// # Note
    ///
    /// This method is a shorthand for using [LexerExt::map_with_lexer] and a callback that returns (`Token`, [Span])
    /// tuples. If you'd like to use a different span type, or wish to perform any other sort of processing, you should
    /// use the [LexerExt::map_with_lexer] method directly.
    ///
    /// [LexerExt::map_with_lexer]: crate::LexerExt::map_with_lexer
    ///
    /// # Example
    ///
    /// ```
    /// use logos::{Logos, UnknownToken};
    ///
    /// #[derive(Logos, Debug, PartialEq)]
    /// enum Example {
    ///     #[regex(r"[ \n\t\f]+", logos::skip)]
    ///     Whitespace,
    ///
    ///     #[regex("-?[0-9]+", |lex| lex.slice().parse().ok())]
    ///     Integer(i64),
    ///
    ///     #[regex("-?[0-9]+\\.[0-9]+", |lex| lex.slice().parse().ok())]
    ///     Float(f64),
    /// }
    ///
    /// let tokens: Vec<_> = Example::lexer("42 3.14 -5 f").spanned().collect();
    ///
    /// assert_eq!(
    ///     tokens,
    ///     &[
    ///         Ok((Example::Integer(42), 0..2)),
    ///         Ok((Example::Float(3.14), 3..7)),
    ///         Ok((Example::Integer(-5), 8..10)),
    ///         Err(UnknownToken), // 'f' is not a recognized token
    ///     ],
    /// );
    /// ```
    #[inline]
    pub fn spanned(self) -> MapWithLexer<'source, Self, SpanFn<'source, Token>> {
        use crate::LexerExt;

        self.map_with_lexer(|result, lexer| result.map(|token| (token, lexer.span())))
    }

    #[inline]
    #[doc(hidden)]
    #[deprecated(since = "0.11.0", note = "please use `span` instead")]
    pub fn range(&self) -> Span {
        self.span()
    }

    /// The source position of the current token.
    #[inline]
    pub fn span(&self) -> Span {
        self.token_start..self.token_end
    }

    /// The source that tokens are being read from. The return type of this method is determined by [Logos::Source], and
    /// will be [&str][str] for most lexers.
    #[inline]
    pub fn source(&self) -> &'source Token::Source {
        self.source
    }

    /// A slice containing the current token. The return type of this method is determined by [Logos::Source], and will
    /// be [&str][str] for most lexers.
    #[inline]
    pub fn slice(&self) -> &'source <Token::Source as Source>::Slice {
        unsafe { self.source.slice_unchecked(self.span()) }
    }

    /// A slice containing the remaining source. This is similar to [Lexer::source], but starts  at the end of the
    /// current token. The return type of this method is determined by [Logos::Source], and will be [&str][str] for
    /// most lexers.
    #[inline]
    pub fn remainder(&self) -> &'source <Token::Source as Source>::Slice {
        unsafe {
            self.source
                .slice_unchecked(self.token_end..self.source.len())
        }
    }

    /// Create a new error value representing a generic "unknown token" error.
    ///
    /// This is a convenience method intended for use within lexer callbacks. You can customise the behaviour of this
    /// method using the [Error] trait and the `error` option in the [Logos] derive macro. See its documentation for
    /// more information.
    #[inline]
    pub fn error(&self) -> Token::Error {
        Token::Error::unknown_token(self)
    }

    /// Turn this lexer into a lexer for a new token type.
    ///
    /// The new lexer points at the same span as this one, but the current token will be replaced with an error.
    pub fn morph<Token2>(self) -> Lexer<'source, Token2>
    where
        Token2: Logos<'source, Source = Token::Source>,
        Token::Extras: Into<Token2::Extras>,
    {
        Lexer {
            source: self.source,
            token: ManuallyDrop::new(None),
            extras: self.extras.into(),
            token_start: self.token_start,
            token_end: self.token_end,
        }
    }

    /// Bump the current span by `n` bytes.
    ///
    /// # Panics
    ///
    /// Panics if the new span is beyond the last byte, or breaks any of the source type's other invariants.
    ///
    /// As an example, `&str` must always be valid UTF-8 - so when used as a source type, this method will panic if the new
    /// span would be in the middle of a UTF-8 code point.
    ///
    /// However, some types do not have invariants such as these! If using `&[u8]` as a source type, the only
    /// requirement is that the new span is within bounds.
    pub fn bump(&mut self, n: usize) {
        self.token_end += n;

        assert!(
            self.source.is_boundary(self.token_end),
            "cannot bump to byte {} as it is not a valid index for the source type",
            self.token_end
        )
    }
}

impl<'source, Token> Clone for Lexer<'source, Token>
where
    Token: Logos<'source> + Clone,
    Token::Extras: Clone,
    Token::Error: Clone,
{
    fn clone(&self) -> Self {
        Lexer {
            extras: self.extras.clone(),
            token: self.token.clone(),
            ..*self
        }
    }
}
