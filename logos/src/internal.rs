use std::mem::ManuallyDrop;

use crate::callback::{CallbackResult, Output};
use crate::source::Chunk;
use crate::{Error, Lexer, Logos, Source};

/// Trait used by the [Logos] derive macro.
///
/// # Abandon hope, all ye who enter here
///
/// This trait (and its methods) are not meant to be used outside of the code produced by `#[derive(Logos)]` macro. The
/// functionality here is not covered under any versioning guarantees, and may change at any time!
pub trait LexerInternal<'source> {
    type Token;
    type Error;

    /// Read a chunk at current position.
    fn read<T: Chunk<'source>>(&self) -> Option<T>;

    /// Read a chunk at current position, offset by `n`.
    fn read_at<T: Chunk<'source>>(&self, n: usize) -> Option<T>;

    /// Unchecked read a chunk at current position, offset by `n`.
    unsafe fn read_unchecked<T: Chunk<'source>>(&self, n: usize) -> T;

    /// Test a chunk at current position with a closure.
    fn test<T: Chunk<'source>, F: FnOnce(T) -> bool>(&self, test: F) -> bool;

    /// Test a chunk at current position offset by `n` with a closure.
    fn test_at<T: Chunk<'source>, F: FnOnce(T) -> bool>(&self, n: usize, test: F) -> bool;

    /// Bump the position by `size`.
    fn bump_unchecked(&mut self, size: usize);

    /// Reset `token_start` to `token_end`.
    fn trivia(&mut self);

    /// Set the current token to the appropriate error value, and guarantee that `token_end` is valid for the source
    /// type. In the case of `&str`, we verify that `token_end` is a valid character boundary.
    fn error(&mut self);

    /// Modify lexer state to represent EOF
    fn end(&mut self);

    /// Set the lexer's current token to `token`.
    fn set(&mut self, token: Result<Self::Token, Self::Error>);

    /// Apply the result of a callback, modifying lexer state accordingly.
    fn apply<C, R, F>(&mut self, result: R, constructor: F)
    where
        Self::Token: Logos<'source>,
        R: CallbackResult<'source, C, Self::Token>,
        F: FnOnce(C) -> Self::Token;
}

impl<'source, Token> LexerInternal<'source> for Lexer<'source, Token>
where
    Token: Logos<'source>,
{
    type Token = Token;
    type Error = Token::Error;

    #[inline]
    fn read<C>(&self) -> Option<C>
    where
        C: Chunk<'source>,
    {
        self.source.read(self.token_end)
    }

    #[inline]
    fn read_at<C>(&self, n: usize) -> Option<C>
    where
        C: Chunk<'source>,
    {
        self.source.read(self.token_end + n)
    }

    #[inline]
    unsafe fn read_unchecked<C>(&self, n: usize) -> C
    where
        C: Chunk<'source>,
    {
        self.source.read_unchecked(self.token_end + n)
    }

    #[inline]
    fn test<C, F>(&self, test: F) -> bool
    where
        C: Chunk<'source>,
        F: FnOnce(C) -> bool,
    {
        self.source.read::<C>(self.token_end).map_or(false, test)
    }

    #[inline]
    fn test_at<C, F>(&self, n: usize, test: F) -> bool
    where
        C: Chunk<'source>,
        F: FnOnce(C) -> bool,
    {
        self.source
            .read::<C>(self.token_end + n)
            .map_or(false, test)
    }

    #[inline]
    fn bump_unchecked(&mut self, size: usize) {
        debug_assert!(
            self.token_end + size <= self.source.len(),
            "Bumping out of bounds!"
        );

        self.token_end += size;
    }

    #[inline]
    fn trivia(&mut self) {
        self.token_start = self.token_end;
    }

    #[inline]
    fn error(&mut self) {
        self.token_end = self.source.find_boundary(self.token_end);
        self.set(Err(Token::Error::unknown_token(self)))
    }

    #[inline]
    fn end(&mut self) {
        self.token = ManuallyDrop::new(None);
    }

    #[inline]
    fn set(&mut self, token: Result<Token, Token::Error>) {
        self.token = ManuallyDrop::new(Some(token));
    }

    #[inline]
    fn apply<C, R, F>(&mut self, result: R, constructor: F)
    where
        Self::Token: Logos<'source>,
        R: CallbackResult<'source, C, Self::Token>,
        F: FnOnce(C) -> Self::Token,
    {
        match result.construct(self) {
            Output::Construct(contents) => self.set(Ok(constructor(contents))),
            Output::Emit(token) => self.set(Ok(token)),
            Output::Error(error) => self.set(Err(error)),
            Output::Skip => {
                self.trivia();
                Self::Token::lex(self);
            }
        }
    }
}
