//! Tools for working with lexers as iterators.
//!
//! Rust's standard library already contains a rich variety of functionality geared around [Iterator]s, but a lot of it
//! doesn't fit in nicely with the rest of Logos - for example, the [Iterator] returned by [Iterator::peekable] doesn't
//! give you any access to the *underlying* iterator, so using `peekable` with a [Lexer] becomes very frustrating.
//!
//! This is quite the pain point, and unfortunately it's a bit hard to come up with a good solution.
//! In an attempt to address this, Logos provides the [LexerExt] trait and a few special iterator types. These work
//! together to allow you to access the underlying [Lexer], even when it's wrapped inside another iterator.
//!
//! ```
//! use logos::{Logos, LexerExt};
//!
//! #[derive(Logos, Debug, PartialEq)]
//! enum Token {
//!     #[regex(r"\s", logos::skip)]
//!     Whitespace,
//!
//!     #[regex(r"[1-9]+[0-9]*")]
//!     Integer,
//! }
//!
//! let mut lexer = Token::lexer("1000 900 2 48").lookahead();
//! // We can use the iterator that `lookahead` returns as usual
//! let peeked = lexer.peek();
//! // ... and we can access information from the `Lexer` too
//! let current = lexer.slice();
//! ```
//!
//! Unfortunately, this approach comes with its own limitations. Most notably, you lose the ability to access the underlying
//! [Lexer] after using an iterator adaptor from the standard library, since those types don't implement [LexerExt]
//!

use crate::{Lexer, LexerExt, Logos};
use std::{marker::PhantomData, mem::ManuallyDrop};

// This is where the magic happens.
impl<'source, Token> Iterator for Lexer<'source, Token>
where
    Token: Logos<'source>,
{
    type Item = Result<Token, Token::Error>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.token_start = self.token_end;

        Token::lex(self);

        // This basically treats `self.token` as a temporary field.
        // Since we always immediately return a newly set token here,
        // we don't have to replace it with `None` or manually drop
        // it later.
        unsafe { ManuallyDrop::take(&mut self.token) }
    }
}

// `dyn A + B` isn't supported, so we need to use our own trait representing `A + B` instead.
#[doc(hidden)]
pub trait LexIterator<'source>: LexerExt<'source> + Iterator {}

impl<'source, T> LexIterator<'source> for T where T: LexerExt<'source> + Iterator {}

// This makes our lives a little easier.
type DynLex<'s, T, I> = dyn LexIterator<'s, Token = T, Item = I> + 's;

/// A boxed and type-erased lexer.
///
/// Since this type contains a [Lexer], it implements the [LexerExt] trait, and allows you to access information from
/// the underlying lexer. See the [trait's documentation][LexerExt] for more information.
///
/// This struct is created by the [LexerExt::boxed] method. See its documentation for more details.
#[cfg(feature = "std")]
pub struct BoxedLexer<'source, Token, Item>
where
    Token: Logos<'source>,
{
    pub(crate) inner: Box<DynLex<'source, Token, Item>>,
    pub(crate) unbox: fn(Box<DynLex<'source, Token, Item>>) -> Lexer<'source, Token>,
}

#[cfg(feature = "std")]
impl<'source, Token, Item> BoxedLexer<'source, Token, Item>
where
    Token: Logos<'source>,
{
    pub(crate) fn new<L>(inner: L) -> BoxedLexer<'source, L::Token, L::Item>
    where
        L: LexerExt<'source> + Iterator<Item = Item> + 'source,
    {
        // SAFETY: The only real assumption we're operating on here is that `repr(*mut dyn Trait)` == `repr(*mut
        // ManuallyDrop<dyn Trait>)`. We know that a `Box` has a valid pointer to the inner value, and also know that
        // the pointer cast is safe - it started life as a value of type `L`, and there's no public way to access it, so
        // it can't be tampered with.
        //
        // Likewise, this function is only ever called in one specific method, and adequate precautions are taken there.
        let unbox = |value| unsafe {
            // Our first order of business is getting rid of the box (we don't want to run its destructor) and getting
            // ownership of its contents in the form of a raw pointer.
            let ptr = Box::into_raw(value);

            // Once we've done that, we can read it back as the original type and move the lexer out.
            let iter = std::ptr::read(ptr as *mut L);
            let unboxed = iter.into_lexer();

            // However, we still need to drop the allocated trait object. We don't want to run its destructor since it's
            // now (technically) been moved. As such, we'll create a new box, holding the trait object wrapped in
            // `ManuallyDrop`.
            //
            // Unfortunately, this requires a transmute. `ManuallyDrop` is `repr(transparent)`, so this is okay.
            let me = std::mem::transmute::<_, *mut ManuallyDrop<DynLex<'source, Token, Item>>>(ptr);

            // Then we can drop the box and free the allocation.
            drop(Box::from_raw(me));

            // ... and we're done. Thank god.
            unboxed
        };

        BoxedLexer {
            inner: Box::new(inner),
            unbox,
        }
    }
}

#[cfg(feature = "std")]
impl<'source, Token, Item> Iterator for BoxedLexer<'source, Token, Item>
where
    Token: Logos<'source>,
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// An iterator that maps each value to another, making use of the lexer in the process.
///
/// Since this type contains a [Lexer], it implements the [LexerExt] trait, and allows you to access information from
/// the underlying lexer. See the [trait's documentation][LexerExt] for more information.
///
/// This struct is created by the [LexerExt::map_with_lexer] method. See its documentation for more details.
pub struct MapWithLexer<'source, L, F> {
    pub(crate) inner: L,
    op: F,
    phantom: PhantomData<&'source ()>,
}

impl<'source, L, F> MapWithLexer<'source, L, F>
where
    L: LexerExt<'source> + Iterator,
{
    pub(crate) fn new(inner: L, op: F) -> Self {
        Self {
            inner,
            op,
            phantom: PhantomData,
        }
    }
}

impl<'source, L, F, O> Iterator for MapWithLexer<'source, L, F>
where
    L: LexerExt<'source> + Iterator,
    F: FnMut(L::Item, &Lexer<'source, L::Token>) -> O,
{
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.inner.next()?;
        let result = (self.op)(value, self.inner.as_lexer());

        Some(result)
    }
}

/// An iterator with a `peek()` method that can look into the future.
///
/// Since this type contains a [Lexer], it implements the [LexerExt] trait, and allows you to access information from
/// the underlying lexer. See the [trait's documentation][LexerExt] for more information.
///
/// This struct is created by the [LexerExt::lookahead] method. See its documentation for more information.
pub struct Lookahead<'source, L>
where
    L: LexerExt<'source> + Iterator,
{
    pub(crate) inner: L,
    peeked: Option<Option<L::Item>>,
    phantom: PhantomData<&'source ()>,
}

// The actual source code here is taken nearly verbatim from the Rust standard library, and is licensed under the MIT
// license or Apache 2.0 license, at your option. The relevant notices can be found at
// https://www.rust-lang.org/policies/licenses, and are additionally included with your Rust distribution. See also the
// LICENSE-MIT and LICENSE-APACHE files.
impl<'source, L> Lookahead<'source, L>
where
    L: LexerExt<'source> + Iterator,
{
    pub(crate) fn new(inner: L) -> Self {
        Self {
            inner,
            peeked: None,
            phantom: PhantomData,
        }
    }

    /// Returns a reference to the next token, without advancing the lexer.
    ///
    /// If the lexer has reached the end of its input, this returns `None`. Otherwise, it returns the token wrapped in
    /// `Some`.
    ///
    /// # Note
    ///
    /// In order to peek at the next token, this method must advance the underlying lexer once. As such, side effects
    /// that the lexer performs - such as mutating the `extras` value - may also be performed when you call this method!
    ///
    /// Likewise, this means that information provided by the lexer - such as the remaining source, and the source
    /// position of the current token - will also be updated.
    #[inline]
    pub fn peek(&mut self) -> Option<&L::Item> {
        let iter = &mut self.inner;

        self.peeked.get_or_insert_with(|| iter.next()).as_ref()
    }

    /// Returns a mutable reference to the next token, without advancing the lexer.
    ///
    /// If the lexer has reached the end of its input, this returns `None`. Otherwise, it returns the token wrapped in
    /// `Some`.
    ///
    /// # Note
    ///
    /// This method has a similar disclaimer to [Lookahead::peek]: In order to peek at the next token, this method must
    /// advance the underlying lexer once. As such, side effects that the lexer performs - such as mutating the `extras`
    /// value - may also be performed when you call this method!
    ///
    /// Likewise, this means that information provided by the lexer - such as the remaining source, and the source
    /// position of the current token - will also be updated.
    #[inline]
    pub fn peek_mut(&mut self) -> Option<&mut L::Item> {
        let iter = &mut self.inner;

        self.peeked.get_or_insert_with(|| iter.next()).as_mut()
    }

    /// Advance the lexer and return the next token, but only if a condition is true.
    ///
    /// If calling `func` on the next token returns `true`, consume and return it.
    /// Otherwise, return `None`.
    pub fn next_if(&mut self, func: impl FnOnce(&L::Item) -> bool) -> Option<L::Item> {
        match self.next() {
            Some(matched) if func(&matched) => Some(matched),
            other => {
                assert!(
                    self.peeked.replace(other).is_none(),
                    "calling `self.next()` should consume the stored `peeked` value"
                );

                None
            }
        }
    }

    /// Advance the lexer and return the next token, but only if it is equal to `expected`.
    pub fn next_if_eq<T>(&mut self, expected: &T) -> Option<L::Item>
    where
        T: ?Sized,
        L::Item: PartialEq<T>,
    {
        self.next_if(|next| next == expected)
    }
}

impl<'source, L> Iterator for Lookahead<'source, L>
where
    L: LexerExt<'source> + Iterator,
{
    type Item = L::Item;

    #[inline]
    fn next(&mut self) -> Option<L::Item> {
        match self.peeked.take() {
            Some(v) => v,
            None => self.inner.next(),
        }
    }

    #[inline]
    fn count(mut self) -> usize {
        match self.peeked.take() {
            Some(None) => 0,
            Some(Some(_)) => 1 + self.inner.count(),
            None => self.inner.count(),
        }
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<L::Item> {
        match self.peeked.take() {
            Some(None) => None,
            Some(v @ Some(_)) if n == 0 => v,
            Some(Some(_)) => self.inner.nth(n - 1),
            None => self.inner.nth(n),
        }
    }

    #[inline]
    fn last(mut self) -> Option<L::Item> {
        let peek_opt = match self.peeked.take() {
            Some(None) => return None,
            Some(v) => v,
            None => None,
        };
        self.inner.last().or(peek_opt)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let peek_len = match self.peeked {
            Some(None) => return (0, Some(0)),
            Some(Some(_)) => 1,
            None => 0,
        };
        let (lo, hi) = self.inner.size_hint();
        let lo = lo.saturating_add(peek_len);
        let hi = match hi {
            Some(x) => x.checked_add(peek_len),
            None => None,
        };
        (lo, hi)
    }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
    where
        Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        let acc = match self.peeked {
            Some(None) => return init,
            Some(Some(v)) => fold(init, v),
            None => init,
        };
        self.inner.fold(acc, fold)
    }
}

// This concludes the source taken from the Rust standard library.
