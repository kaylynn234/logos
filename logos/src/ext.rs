use crate::{
    iter::{BoxedLexer, Lookahead, MapWithLexer},
    Lexer, Logos, Span,
};

type Source<'s, T> = <T as Logos<'s>>::Source;
type Slice<'s, T> = <Source<'s, T> as crate::Source>::Slice;
type Extras<'s, T> = <T as Logos<'s>>::Extras;

/// Extension methods for types that contain a [Lexer].
pub trait LexerExt<'source> {
    /// The token type that the [Lexer] uses.
    type Token: Logos<'source>;

    /// Get a reference to the underlying [Lexer].
    fn as_lexer(&self) -> &Lexer<'source, Self::Token>;

    /// Get a mutable reference to the underlying [Lexer].
    fn as_lexer_mut(&mut self) -> &mut Lexer<'source, Self::Token>;

    /// Consume `self`, and return the underlying [Lexer].
    fn into_lexer(self) -> Lexer<'source, Self::Token>;

    /// The source position of the current token. This is equivalent to [Lexer::span].
    #[inline(always)]
    fn span(&self) -> Span {
        self.as_lexer().span()
    }

    /// The source that tokens are being read from. The return type of this method is determined by [Logos::Source], and
    /// will be [`&str`][str] for most lexers. This is equivalent to [Lexer::source].
    #[inline(always)]
    fn source(&self) -> &'source Source<'source, Self::Token> {
        self.as_lexer().source()
    }

    /// A slice containing the current token. The return type of this method is determined by [Logos::Source], and will
    /// be [`&str`][str] for most lexers. This is equivalent to [Lexer::slice].
    #[inline(always)]
    fn slice(&self) -> &'source Slice<'source, Self::Token> {
        self.as_lexer().slice()
    }

    /// A slice containing the remaining source. This is similar to [Lexer::source], but starts  at the end of the
    /// current token. The return type of this method is determined by [Logos::Source], and will be [&str][str] for
    /// most lexers. This is equivalent to [Lexer::remainder].
    #[inline(always)]
    fn remainder(&self) -> &'source Slice<'source, Self::Token> {
        self.as_lexer().remainder()
    }

    /// Get a reference to the lexer's extras. This is a shorthand for calling [LexerExt::as_lexer] and accessing the
    /// `extras` field.
    #[inline(always)]
    fn extras<'lexer>(&'lexer self) -> &'lexer Extras<'source, Self::Token>
    where
        'source: 'lexer,
    {
        &self.as_lexer().extras
    }

    /// Get a mutable reference to the lexer's extras. This is a shorthand for calling [LexerExt::as_lexer_mut] and
    /// accessing the `extras` field.
    fn extras_mut<'lexer>(&'lexer mut self) -> &'lexer mut Extras<'source, Self::Token>
    where
        'source: 'lexer,
    {
        &mut self.as_lexer_mut().extras
    }

    /// Wrap the lexer in an [Iterator] that maps each token to another value, making use of the lexer in the process.
    ///
    /// The returned iterator produces values by calling `op` for each token, passing both the token and a reference to
    /// the lexer as arguments.
    ///
    /// See also [Lexer::spanned], which uses this method to pair tokens with their source positions.
    #[inline]
    fn map_with_lexer<F, O>(self, op: F) -> MapWithLexer<'source, Self, F>
    where
        Self: Sized + Iterator,
        F: FnMut(Self::Item, &Lexer<'source, Self::Token>) -> O,
    {
        MapWithLexer::new(self, op)
    }

    /// Box the lexer, returning a type-erased [BoxedLexer].
    ///
    /// This incurs a small performance penalty from dynamic dispatch, but makes it possible to name the type of the
    /// lexer when you might otherwise be unable to.
    #[inline]
    fn boxed(self) -> BoxedLexer<'source, Self::Token, Self::Item>
    where
        Self: Sized + Iterator + 'source,
    {
        // Silly inference quirk.
        BoxedLexer::<Self::Token, Self::Item>::new(self)
    }

    /// Wrap the [Lexer] in an [Iterator] that can use the [peek][Lookahead::peek] and [peek_mut][Lookahead::peek_mut]
    /// methods to see the future.
    ///
    ///
    #[inline]
    fn lookahead(self) -> Lookahead<'source, Self>
    where
        Self: Sized + Iterator + 'source,
    {
        Lookahead::new(self)
    }
}

impl<'source, Token> LexerExt<'source> for Lexer<'source, Token>
where
    Token: Logos<'source>,
{
    type Token = Token;

    #[inline(always)]
    fn as_lexer(&self) -> &Lexer<'source, Self::Token> {
        self
    }

    #[inline(always)]
    fn as_lexer_mut(&mut self) -> &mut Lexer<'source, Self::Token> {
        self
    }

    #[inline(always)]
    fn into_lexer(self) -> Lexer<'source, Self::Token> {
        self
    }
}

impl<'source, Token, Item> LexerExt<'source> for BoxedLexer<'source, Token, Item>
where
    Token: Logos<'source>,
{
    type Token = Token;

    #[inline]
    fn as_lexer(&self) -> &Lexer<'source, Self::Token> {
        self.inner.as_lexer()
    }

    #[inline]
    fn as_lexer_mut(&mut self) -> &mut Lexer<'source, Self::Token> {
        self.inner.as_lexer_mut()
    }

    #[inline]
    fn into_lexer(self) -> Lexer<'source, Self::Token> {
        (self.unbox)(self.inner)
    }
}

impl<'source, L, F> LexerExt<'source> for MapWithLexer<'source, L, F>
where
    L: LexerExt<'source>,
{
    type Token = L::Token;

    #[inline]
    fn as_lexer(&self) -> &Lexer<'source, Self::Token> {
        self.inner.as_lexer()
    }

    #[inline]
    fn as_lexer_mut(&mut self) -> &mut Lexer<'source, Self::Token> {
        self.inner.as_lexer_mut()
    }

    #[inline]
    fn into_lexer(self) -> Lexer<'source, Self::Token> {
        self.inner.into_lexer()
    }
}

impl<'source, L> LexerExt<'source> for Lookahead<'source, L>
where
    L: LexerExt<'source> + Iterator,
{
    type Token = L::Token;

    #[inline]
    fn as_lexer(&self) -> &Lexer<'source, Self::Token> {
        self.inner.as_lexer()
    }

    #[inline]
    fn as_lexer_mut(&mut self) -> &mut Lexer<'source, Self::Token> {
        self.inner.as_lexer_mut()
    }

    #[inline]
    fn into_lexer(self) -> Lexer<'source, Self::Token> {
        self.inner.into_lexer()
    }
}
