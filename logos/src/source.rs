//! Traits for reading from different input sources.
//!
//! The most notable trait in this module is [Source], which represents a source type that a [Lexer][crate::Lexer] can
//! read from. You can use your own source type by passing a type to the derive macro's `source` option. See the
//! documentation on the [Logos][crate::Logos] trait for more information.
//!
//! This module also contains the [Chunk] trait, for types that represent a fixed-size "chunk" of bytes. [Chunk] is
//! mainly used internally, so it's unlikely that you'll have much reason to implement it yourself.

use std::fmt::Debug;
use std::ops::Range;

/// Types the `Lexer` can read from.
///
/// This trait is already implemented for `&str` and `&[u8]`, so it's unlikely that you'll want to implement it
/// yourself.
///
/// That being said, you can pass a type implementing this trait to the derive macro's `source` option to use your own
/// source type. See the documentation on the [Logos][crate::Logos] trait for more information.
pub trait Source {
    /// A slice of this `Source`
    type Slice: ?Sized + PartialEq + Eq + Debug;

    /// The length of the source, in bytes.
    fn len(&self) -> usize;

    /// Whether the source is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Read a chunk of bytes into an array. Returns `None` when reading
    /// out of bounds would occur.
    ///
    /// This is very useful if you'd like to perform comparisons with fixed-size byte arrays, and tends to be quite fast
    /// since the array's size is statically known.
    ///
    /// ```rust
    /// use logos::Source;
    ///
    /// let foo = "foo";
    ///
    /// assert_eq!(foo.read::<u8>(2), Some(b'o'));
    /// assert_eq!(foo.read::<&[u8; 3]>(0), Some(b"foo"));
    /// assert_eq!(foo.read::<&[u8; 2]>(0), Some(b"fo"));
    /// assert_eq!(foo.read::<&[u8; 4]>(0), None); // Out of bounds
    /// assert_eq!(foo.read::<&[u8; 2]>(2), None); // Out of bounds
    /// ```
    fn read<'a, Chunk>(&'a self, offset: usize) -> Option<Chunk>
    where
        Chunk: self::Chunk<'a>;

    /// Read a chunk of bytes into an array, without performing bounds checks.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `offset + Chunk::SIZE` does not exceed `self.len`, otherwise calling this method
    /// is **immediate undefined behaviour**.
    unsafe fn read_unchecked<'a, Chunk>(&'a self, offset: usize) -> Chunk
    where
        Chunk: self::Chunk<'a>;

    /// Return the slice of input corresponding to `range`, or `None` if `range` is out of bounds.
    ///
    /// This is analogous to `slice::get(range)`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use logos::Source;
    ///
    /// let foo = "It was the year when they finally immanentized the Eschaton.";
    /// assert_eq!(<str as Source>::slice(&foo, 51..59), Some("Eschaton"));
    /// ```
    fn slice(&self, range: Range<usize>) -> Option<&Self::Slice>;

    /// Return the slice of input corresponding to `range`, **without performing any bounds checks**.
    ///
    /// This is analogous to `slice::get_unchecked(range)`.
    ///
    /// # Safety
    ///
    /// Similarly to `slice::get_unchecked(range)`, the caller must ensure that the provided range is within bounds.
    /// **Providing an out-of-bounds range is immediate undefined behaviour**, even if the return value of this method
    /// is never used.
    ///
    /// # Example
    ///
    /// ```rust
    /// use logos::Source;
    ///
    /// let foo = "It was the year when they finally immanentized the Eschaton.";
    ///
    /// unsafe {
    ///     assert_eq!(<str as Source>::slice_unchecked(&foo, 51..59), "Eschaton");
    /// }
    /// ```
    unsafe fn slice_unchecked(&self, range: Range<usize>) -> &Self::Slice;

    /// Find the closest valid index for this `Source`, starting at `index`.
    ///
    /// For binary sources, this is usually not a concern - it's perfectly okay to use an arbitrary index, as long as
    /// it's within bounds. However, this may not be the case for other sources! For example, Rust's `&str` type can
    /// only be indexed using  *valid UTF-8 character boundaries*, because of the invariant that all strings are valid
    /// UTF-8. When implementing your own `Source`, this method can be used to ensure similar invariants, if necessary.
    ///
    /// The default implementation returns `index` as-is.
    #[inline]
    fn find_boundary(&self, index: usize) -> usize {
        index
    }

    /// Check if `index` is valid for this `Source`. Namely, ensure that
    ///
    /// * `index` is not larger than the byte length of this `Source`.
    /// * `index` would not violate any of this type's invariants. For example, `str` must make sure that `index` is a
    ///   valid UTF-8 codepoint boundary.
    fn is_boundary(&self, index: usize) -> bool;
}

impl Source for str {
    type Slice = str;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn read<'a, C>(&'a self, offset: usize) -> Option<C>
    where
        C: Chunk<'a>,
    {
        if offset + C::SIZE <= self.len() {
            Some(unsafe { C::from_ptr(self.as_ptr().add(offset)) })
        } else {
            None
        }
    }

    #[inline]
    unsafe fn read_unchecked<'a, C>(&'a self, offset: usize) -> C
    where
        C: Chunk<'a>,
    {
        C::from_ptr(self.as_ptr().add(offset))
    }

    #[inline]
    fn slice(&self, range: Range<usize>) -> Option<&str> {
        self.get(range)
    }

    #[inline]
    unsafe fn slice_unchecked(&self, range: Range<usize>) -> &str {
        debug_assert!(
            range.start <= self.len() && range.end <= self.len(),
            "Reading out of bounds {:?} for {}!",
            range,
            self.len()
        );

        self.get_unchecked(range)
    }

    #[inline]
    fn find_boundary(&self, mut index: usize) -> usize {
        while !self.is_char_boundary(index) {
            index += 1;
        }

        index
    }

    #[inline]
    fn is_boundary(&self, index: usize) -> bool {
        self.is_char_boundary(index)
    }
}

impl Source for [u8] {
    type Slice = [u8];

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn read<'a, C>(&'a self, offset: usize) -> Option<C>
    where
        C: Chunk<'a>,
    {
        if offset + C::SIZE <= self.len() {
            Some(unsafe { C::from_ptr(self.as_ptr().add(offset)) })
        } else {
            None
        }
    }

    #[inline]
    unsafe fn read_unchecked<'a, C>(&'a self, offset: usize) -> C
    where
        C: Chunk<'a>,
    {
        C::from_ptr(self.as_ptr().add(offset))
    }

    #[inline]
    fn slice(&self, range: Range<usize>) -> Option<&[u8]> {
        self.get(range)
    }

    #[inline]
    unsafe fn slice_unchecked(&self, range: Range<usize>) -> &[u8] {
        debug_assert!(
            range.start <= self.len() && range.end <= self.len(),
            "Reading out of bounds {:?} for {}!",
            range,
            self.len()
        );

        self.get_unchecked(range)
    }

    #[inline]
    fn is_boundary(&self, index: usize) -> bool {
        index <= self.len()
    }
}

/// A fixed, statically sized chunk of data that can be read from a `Source`.
///
/// This is implemented for `u8`, as well as borrowed byte arrays of any size.
pub trait Chunk<'source>: Sized + Copy + PartialEq + Eq {
    /// The size of the chunk, in bytes.
    const SIZE: usize;

    /// Create a chunk from a raw pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `ptr` is well-aligned, non-dangling and properly initialised for a value of type
    /// `Self`. Additionally, the caller must ensure that `ptr` is valid for reads of at least `Self::SIZE` bytes.
    ///
    /// Violating any of these conditions is immediate undefined behaviour.
    unsafe fn from_ptr(ptr: *const u8) -> Self;
}

impl<'source> Chunk<'source> for u8 {
    const SIZE: usize = 1;

    #[inline]
    unsafe fn from_ptr(ptr: *const u8) -> Self {
        *ptr
    }
}

impl<'source, const SIZE: usize> Chunk<'source> for &'source [u8; SIZE] {
    const SIZE: usize = SIZE;

    #[inline]
    unsafe fn from_ptr(ptr: *const u8) -> Self {
        &*(ptr as *const [u8; SIZE])
    }
}
