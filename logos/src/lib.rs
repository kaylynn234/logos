//! <img src="https://raw.githubusercontent.com/maciejhirsz/logos/master/logos.svg?sanitize=true" alt="Logos logo" width="250" align="right">
//!
//! # Logos
//!
//! _Create ridiculously fast lexers_
//!
//! Logos has two goals:
//!
//! - To make it easy for you to create a lexer, so that you can spend your time on more complex problems.
//! - To make that lexer faster than anything you'd write by hand.
//!
//! To achieve this, Logos:
//!
//! - Combines all token definitions into a single [deterministic state machine](https://en.wikipedia.org/wiki/Deterministic_finite_automaton).
//! - Optimizes branches into [lookup tables](https://en.wikipedia.org/wiki/Lookup_table) or [jump tables](https://en.wikipedia.org/wiki/Branch_table).
//! - Prevents [backtracking](https://en.wikipedia.org/wiki/ReDoS) inside token definitions.
//! - [Unwinds loops](https://en.wikipedia.org/wiki/Loop_unrolling), and batches reads to minimize bounds checking.
//! - Does all of that heavy lifting at compile time!
//!
//! ## Example
//!
//! ```rust
//! use logos::Logos;
//!
//! #[derive(Logos, Debug, PartialEq)]
//! enum Token {
//!     // Tokens can be literal strings, of any length.
//!     #[token("fast")]
//!     Fast,
//!
//!     #[token(".")]
//!     Period,
//!
//!     // Or regular expressions.
//!     #[regex("[a-zA-Z]+")]
//!     Text,
//!
//!     // We can also use this variant to define whitespace,
//!     // or any other matches we wish to skip.
//!     #[regex(r"[ \t\n\f]+", logos::skip)]
//!     Whitespace,
//! }
//!
//! fn main() {
//!     let mut lex = Token::lexer("Create ridiculously fast lexers.");
//!
//!     assert_eq!(lex.next(), Some(Ok(Token::Text)));
//!     assert_eq!(lex.span(), 0..6);
//!     assert_eq!(lex.slice(), "Create");
//!
//!     assert_eq!(lex.next(), Some(Ok(Token::Text)));
//!     assert_eq!(lex.span(), 7..19);
//!     assert_eq!(lex.slice(), "ridiculously");
//!
//!     assert_eq!(lex.next(), Some(Ok(Token::Fast)));
//!     assert_eq!(lex.span(), 20..24);
//!     assert_eq!(lex.slice(), "fast");
//!
//!     assert_eq!(lex.next(), Some(Ok(Token::Text)));
//!     assert_eq!(lex.span(), 25..31);
//!     assert_eq!(lex.slice(), "lexers");
//!
//!     assert_eq!(lex.next(), Some(Ok(Token::Period)));
//!     assert_eq!(lex.span(), 31..32);
//!     assert_eq!(lex.slice(), ".");
//!
//!     assert_eq!(lex.next(), None);
//! }
//! ```
//!
//! ## Callbacks
//!
//! Logos can also call arbitrary functions whenever a pattern is matched, which can be used to put data into a variant or
//! perform more complicated lexing if regular expressions are too limiting.
//!
//! ```rust
//! use logos::{Logos, Lexer};
//!
//! fn kilo(lex: &mut Lexer<Token>) -> Option<u64> {
//!     let slice = lex.slice();
//!     let n: u64 = slice[..slice.len() - 1].parse().ok()?; // skip 'k'
//!     
//!     Some(n * 1_000)
//! }
//!
//! fn mega(lex: &mut Lexer<Token>) -> Option<u64> {
//!     let slice = lex.slice();
//!     let n: u64 = slice[..slice.len() - 1].parse().ok()?; // skip 'm'
//!
//!     Some(n * 1_000_000)
//! }
//!
//! #[derive(Logos, Debug, PartialEq)]
//! enum Token {
//!     #[regex(r"[ \t\n\f]+", logos::skip)]
//!     Whitespace,
//!
//!     // Callbacks can use closure syntax, or refer
//!     // to a function defined elsewhere.
//!     //
//!     // Each pattern can have its own callback.
//!     #[regex("[0-9]+", |lex| lex.slice().parse().ok())]
//!     #[regex("[0-9]+k", kilo)]
//!     #[regex("[0-9]+m", mega)]
//!     Number(u64),
//! }
//!
//! fn main() {
//!     let mut lex = Token::lexer("5 42k 75m");
//!
//!     assert_eq!(lex.next(), Some(Ok(Token::Number(5))));
//!     assert_eq!(lex.slice(), "5");
//!
//!     assert_eq!(lex.next(), Some(Ok(Token::Number(42_000))));
//!     assert_eq!(lex.slice(), "42k");
//!
//!     assert_eq!(lex.next(), Some(Ok(Token::Number(75_000_000))));
//!     assert_eq!(lex.slice(), "75m");
//!
//!     assert_eq!(lex.next(), None);
//! }
//! ```
//!
//! Callbacks can return any value implementing the [CallbackResult][crate::callback::CallbackResult] trait. This trait is implemented already for `Option`,
//! `Result`, `bool` and many more - see the [documentation on callbacks][./index.html] for more information.
//!
//! ## Token disambiguation
//!
//! The rule of thumb is:
//! - Longer beats shorter.
//! - Specific beats generic.
//!
//! If any two patterns could match the same input, such as `fast` and `[a-zA-Z]+` from the example above, **Logos will
//! choose the longer and more specific pattern**. In this case, that means that a string like "fast" will result in
//! `Token::Fast` instead of `Token::Text`.
//!
//! This is done by comparing the *numeric priority* of each definition. This priority is calculated using a
//! simple set of rules:
//! - A regex *character class* (like `[a-z]`) has a priority of 1.
//! - A byte of text has a priority of 2.
//!
//! The priority of a repetition or alternation is the priority of its **shortest possible match**. More specifically,
//! - The shortest possible match for patterns like `term*` and `term?` is an empty string.
//! - The shortest possible match for a pattern like `term+` is just `term`, since `+` matches *at least* once.
//! - The shortest possible match for an alternation is always the shortest alternative.
//!
//! For example,
//! - `[a-zA-Z]+` has a priority of 1; the priority of `[a-zA-Z]` is 1, and the `+` quantifier doesn't affect the shortest
//!   possible match.
//! - `foobar` has a priority of 12; it's 6 bytes, and each byte has a priority of 2.
//! - `(foo|hello)(bar)?` has a priority of 6. `foo` is the shortest option in the alternation, with a priority of 6. The
//!   shortest possible match for `(bar)?` is an empty string, so it has a priority of 0.
//!
//! If two definitions have the same priority and might match the same input, you'll receive a compile error! Logos will
//! point out the problematic definitions, and ask you to manually specify a priority to fix the issue.
//!
//! For example, the patterns `[abc]+` and `[cde]+` both have a priority of 1, and will *both* match repeated `c`s. You can
//! fix this ambiguity by changing the definition to `#[regex("[abc]+", priority = 2)]` within the derive macro. In this
//! case, the definition using `"[abc]+"` now has a higher priority, so all sequences of `c` will match the `[abc]+`
//! definition instead.
//!
//! ## How fast?
//!
//! Ridiculously fast!
//!
//! ```norust
//! test identifiers                        ... bench:         647 ns/iter (+/- 27) = 1204 MB/s
//! test keywords_operators_and_punctuators ... bench:       2,054 ns/iter (+/- 78) = 1037 MB/s
//! test strings                            ... bench:         553 ns/iter (+/- 34) = 1575 MB/s
//! ```
//!
//! ### What about the changes you made?
//!
//! I ran benchmarks on both this fork and the upstream repository, and rest assured that even *after* my changes, Logos is
//! still just as fast. Hooray!
//!
//! ## Acknowledgements
//!
//! - [Pedrors](https://pedrors.pt/) for the **Logos** logo.
//!
//! ## Thank you
//!
//! Logos is very much a labor of love, so consider [buying the *real* author a coffee](https://github.com/sponsors/maciejhirsz). ☕
//!
//! ## License
//!
//! Logos is licensed under the terms of both the MIT license and the Apache License (Version 2.0), at your option.
//!
//! See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
//!
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://maciej.codes/kosz/logos.png")]

#[cfg(not(feature = "std"))]
extern crate core as std;

#[doc(hidden)]
pub mod internal;

pub mod callback;
pub mod error;
mod ext;
pub mod iter;
mod lexer;
pub mod source;

pub use crate::error::{Error, UnknownToken};
pub use crate::ext::LexerExt;
pub use crate::lexer::{Lexer, Span};
pub use crate::source::Source;
#[cfg(feature = "export_derive")]
pub use logos_derive::Logos;

/// Types that can be lexed using [Lexer].
///
/// The section below is quite long, so here's a handy link to [jump to the rest of the trait
/// documentation](#associated-types).
///
/// # Using the derive macro
///
/// The main way to interact with this crate is through `#[derive(Logos)]` - the [Logos] derive macro. The derive macro is
/// responsible for generating an implementation of this trait, and it's what does all the heavy lifting for you. A
/// lexer would be useless if you couldn't tell it *what* and *how* to lex, so the derive macro also accepts a healthy
/// set of options to configure the generated lexer.
///
/// ## Attributes
///
/// The derive macro uses [attributes](https://doc.rust-lang.org/reference/attributes.html) to customize the generated
/// [Logos] implementation. For the purposes of documentation, these are split into two categories:
/// * **Enum attributes**, which are applied to an enum definition
/// * **Variant attributes**, which are applied to an individual enum variant
///
/// It's important to note that a single enum or enum variant can have multiple attributes applied to it.
///
/// ## Enum attributes
///
/// ### `#[logos(error = SomeType)]`
///
/// Sets the [error type][Logos::Error] for this [Logos] implementation.
///
/// Logos will use this type to report lexing errors - namely, encountering an unknown token - but you can also use this
/// type within callbacks to emit more detailed errors.
///
/// The error type is [UnknownToken] by default.
///
/// See the [documentation on callbacks](./callback/index.html) for details not covered here.
///
/// ### `#[logos(extras = SomeType)]`
///
/// Sets the [extras type][Logos::Extras] for this [Logos] implementation.
///
/// The [Lexer] will store a value of this type in its `extras` field, and you're free to modify it as you wish. Because
/// callbacks are passed `&mut Lexer` as an argument, this value can also be accessed and modified within callbacks, so
/// it's useful if you'd like to store additional state within your lexer.
///
/// The extras type is `()` by default.
///
/// See the documentation on [callbacks](./callback/index.html), the [extras type][Logos::Extras] and the [Lexer] type
/// for details not covered here.
///
/// ### `#[logos(type T = SomeType)]`
///
/// Specify the concrete type to use for the type parameter `T`.
///
/// If your lexer contains generic type parameters, Logos will emit an error - it doesn't know what type should be used
/// to fill in those type parameters. As an example, the following snippet fails to compile:
///
/// ```compile_fail
/// use logos::{Logos, Lexer};
///
/// // A magical type!
/// #[derive(Default)]
/// struct SuperMagic;
///
/// // A callback that produces a default value for `T`.
/// fn make_magic<'source, T>(lexer: &mut Lexer<'source, Token<T>>) -> T
/// where
///     T: Default + 'source,
///     Token<T>: Logos<'source>
/// {
///     T::default()
/// }
///
/// #[derive(Logos)]
/// enum Token<T: Default> {
///     #[token("magic", make_magic)]
///     Magic(T)
/// }
/// ```
///
/// However, we can use the `type` option to tell Logos what type `T` (or any other type parameter) should be:
///
/// ```
/// # use logos::{Logos, Lexer};
/// #
/// # #[derive(Default)]
/// # struct SuperMagic;
/// #
/// # fn make_magic<'source, T>(lexer: &mut Lexer<'source, Token<T>>) -> T
/// # where
/// #     T: Default + 'source,
/// #     Token<T>: Logos<'source>
/// # {
/// #     T::default()
/// # }
/// #
/// #[derive(Logos)]
/// #[logos(type T = SuperMagic)] // This is important!
/// enum Token<T: Default> {
///     #[token("magic", make_magic)]
///     Magic(T)
/// }
/// ```
///
/// At present, the derive macro does not perform *generic implementations* of the [Logos] trait, so you must always
/// specify replacements for type parameters. This is likely to change in the future.
///
/// ### `#[logos(subpattern NAME = "...")]`
///
/// Define a subpattern named `NAME` that can be used within regular expressions.
///
/// Subpatterns allow you to avoid code duplication by factoring out the common parts of a regular expression, and
/// additionally help readability by making it possible to "name" a regular expression's component parts.
///
/// Inside of regular expressions, subpatterns are used by wrapping `?&` and a *subpattern name* within parenthesis. For
/// example, the expression `(?&IDENTIFIER)` refers to a subpattern named `IDENTIFIER`.
///
/// ## Variant attributes
///
/// ### `#[token(...)]` and `#[regex(...)]`
///
/// The `#[token(...)]` and `#[regex(...)]` attributes are used to add rules to your lexer, so that you can produce
/// tokens based on certain criteria. Both of these attributes are very similar (and share common configuration options)
/// so they're grouped here for easier reading.
///  
/// #### `#[token("LITERAL", ...)]`
///
/// Produce this variant when the provided literal string is found.
///
/// A `#[token(...)]` definition should be used for tokens that are represented *solely* by a specific piece of input,
/// such as keywords or symbols. For example,
///
/// ```
/// use logos::Logos;
///
/// #[derive(Logos)]
/// enum Token {
///     #[token("if")]
///     If,
///     #[token("then")]
///     Then,
///     #[token("else")]
///     Else,
///     // ... and so on. You can use any string you'd like!
/// }
/// ```
///
/// For tokens that may be represented by *any* input that fits a certain criteria (such as string literals or
/// identifiers) you should use a `#[regex(...)]` definition instead.
///
/// #### `#[regex("PATTERN", ...)]`
///
/// Produce this variant when the provided regular expression matches.
///
/// A `#[regex(...)]` definition should be used for tokens that are represented by *any* input that fits a certain
/// criteria - such as integer literals, identifiers, and similar. Integer literals could be nearly *any* sequence of
/// the numbers 0 through 9, and writing out every possible number using `#[token(...)]` definitions would be a
/// *colossal* waste of time.
///
/// We can use a `#[regex(...)]` definition for this instead, like so:
///
/// ```
/// use logos::Logos;
///
/// #[derive(Logos, Debug, PartialEq)]
/// enum Token {
///     // Callbacks are explained further below, but for now all you need to know is that we use the `logos::skip`
///     // callback here so that we can ignore whitespace.
///     #[regex(r"\s", logos::skip)]
///     Whitespace,
///
///     #[regex(r"[1-9]+[0-9]*")]
///     Integer,
///     // ... and so on. See below for more details on what you can and can't do.
/// }
///
/// let mut lexer = Token::lexer("89 144 233");
///
/// lexer.next();
/// assert_eq!(lexer.slice(), "89");
///
/// lexer.next();
/// assert_eq!(lexer.slice(), "144");
///
/// lexer.next();
/// assert_eq!(lexer.slice(), "233");
///
/// // We've reached the end of our input, so calling `next` again returns `None`
/// assert_eq!(lexer.next(), None);
/// ```
///
/// The regular expression used within a `#[regex(...)]` definition is subject to some limitations. Most notably:
/// - Look-around is not supported
/// - Backreferences are not supported
/// - Line anchors may not be used
/// - Capture groups cannot be used to extract portions of the matched input.
///
/// If you'd like to perform more complicated lexing, you can use *lexer callbacks*, which are described below and in
/// the [documentation on callbacks](./callback/index.html).
///
/// #### Callbacks
///
/// Callbacks may be attached to a `#[token(...)]` or `#[regex(...)]` definition, and are called whenever a match
/// occurs. They can be used to put data into a variant, skip a token match, or to emit errors. Callbacks can also
/// perform more complicated lexing if regular expressions are too limiting.
///
/// Callbacks may be provided as closures, or as paths to functions defined elsewhere.
/// They are typically passed as positional arguments to the `#[token(...)]` or `#[regex(...)]` attributes, like so:
///
/// ```
/// use logos::Logos;
///
/// #[derive(Logos, Debug, PartialEq)]
/// enum Token {
///     // We can use the predefined `skip` callback to ignore whitespace
///     #[regex(r"\s", logos::skip)]
///     Whitespace,
///
///     // ... and we can also use a closure to put data into a variant
///     #[regex("[0-9]+", |lex| lex.slice().parse().ok())]
///     Integer(usize)
/// }
///
/// let mut lexer = Token::lexer("2000 305 8");
///
/// assert_eq!(lexer.next(), Some(Ok(Token::Integer(2000))));
/// // The whitespace between `2000` and `305` is skipped
/// assert_eq!(lexer.next(), Some(Ok(Token::Integer(305))));
/// // The whitespace between `305` and `8` is skipped
/// assert_eq!(lexer.next(), Some(Ok(Token::Integer(8))));
/// // We've reached the end of our input, so the lexer returns `None`
/// assert_eq!(lexer.next(), None);
/// ```
///
/// Callbacks may also be passed using the `callback` option, which is described further in the section below.
///
/// See the [documentation on callbacks](./callback/index.html) for
/// details not covered here.
///
/// #### Other options
///
/// The `#[token(...)]` and `#[regex(...)]` attributes also support additional configuration options.
///
/// **Additional options must come after the pattern and callback**. For example, the following is invalid:
///
/// ```compile_fail
/// use logos::Logos;
///
/// #[derive(Logos)]
/// enum Token {
///     #[token(priority = 20, "fast")]
///     Fast
/// }
/// ```
///
/// ##### `priority = ...`
///
/// Sets the priority of the definition. This argument's value should be an integer.
///
/// The priority value is used to disambiguate tokens, and is normally calculated automatically. Logos will issue a
/// compiler error if any two definitions have the same priority and could match the same input, so this option can be
/// used to solve conflicts in that scenario.
///
/// For example, the following two definitions are in conflict:
///
/// ```compile_fail
/// use logos::Logos;
///
/// #[derive(Logos)]
/// enum Token {
///     #[token("fast")]
///     Fast,
///
///     #[regex("(ridiculously)?fast(er|est)?")]
///     RidiculouslyFast,
/// }
/// ```
/// Both the `#[regex("(ridiculously)?fast[er|est]?")]` and `#[token("fast")]` definitions could match the input "fast",
/// and both have the same priority.
///
/// When you encounter a conflict like this, Logos will also use the compile error to suggest a priority value that
/// isn't ambiguous. In this case, `fast` has a priority of 8, so Logos suggested that we use a priority of 9 to
/// disambiguate:
///
/// ```no_rust
/// error: A definition of variant `RidiculouslyFast` can match the same input as another definition of variant `Fast`.
///
///        hint: Consider giving one definition a higher priority: #[regex(..., priority = 9)]
///   -> src\example.rs:8:13
///   |
/// 5 |     #[regex("fast[er|est]?")]
///   |             ^^^^^^^^^^^^^^^
///
/// error: A definition of variant `Fast` can match the same input as another definition of variant `RidiculouslyFast`.
///
///        hint: Consider giving one definition a higher priority: #[regex(..., priority = 9)]
///  --> src\example.rs:8:13
///   |
/// 8 |     #[token("fast")]
///   |             ^^^^^^
/// ```
///
/// You can then solve this conflict by using the `priority` option:
///
/// ```
/// use logos::Logos;
///
/// #[derive(Logos, Debug, PartialEq)]
/// enum Token {
///     #[regex(r"\s", logos::skip)]
///     Whitespace,
///
///     #[token("fast", priority = 9)]
///     Fast,
///
///     #[regex("(ridiculously)?fast(er|est)?")]
///     RidiculouslyFast,
/// }
///
/// let mut lexer = Token::lexer("fast faster fastest ridiculouslyfast");
///
/// assert_eq!(lexer.next(), Some(Ok(Token::Fast)));
/// assert_eq!(lexer.slice(), "fast");
///
/// assert_eq!(lexer.next(), Some(Ok(Token::RidiculouslyFast)));
/// assert_eq!(lexer.slice(), "faster");
///
/// assert_eq!(lexer.next(), Some(Ok(Token::RidiculouslyFast)));
/// assert_eq!(lexer.slice(), "fastest");
///
/// assert_eq!(lexer.next(), Some(Ok(Token::RidiculouslyFast)));
/// assert_eq!(lexer.slice(), "ridiculouslyfast");
///
/// // We've reached the end of our input, so calling `next` again returns `None`
/// assert_eq!(lexer.next(), None);
/// ```
///
/// ##### `callback = ...`
///
/// Sets the callback attached to this definition. This argument's value should be a closure or a path to a function
/// defined elsewhere.
///
/// For example,
///
/// ```
/// use logos::Logos;
///
/// #[derive(Logos)]
/// enum Token {
///     // We can use the predefined `skip` callback to ignore whitespace
///     #[regex(r"\s", callback = logos::skip)]
///     Whitespace,
/// }
/// ```
///
/// **Callbacks may also be passed as positional arguments**, so you likely won't find yourself needing to use the named
/// form. See the section above and the [documentation on callbacks](./callback/index.html) for details not covered
/// here.
///
/// ##### `ignore(...)`
///
/// Ignore certain aspects of the input when performing comparisons. This argument's value should be a parenthesized
/// sequence of flags.
///
/// Valid flags are
/// * `case` - Comparisons between characters will be entirely **case-insensitive**. This flag may not be used with
///   `ascii_case`.
/// * `ascii_case` - Comparisons between **ASCII** characters will be **case-insensitive**. This flag may not be used
///   with `case`.
///
/// For example:
///
/// ```
/// use logos::Logos;
///
/// #[derive(Logos, Debug, PartialEq)]
/// enum Token {
///     #[regex(r"\s", logos::skip)]
///     Whitespace,
///
///     #[regex("[a-z]+[0-9a-z]*", ignore(ascii_case))]
///     Identifier,
/// }
///
/// let mut lexer = Token::lexer("Case iS UNIMPORtant");
///
/// lexer.next();
/// assert_eq!(lexer.slice(), "Case");
///
/// lexer.next();
/// assert_eq!(lexer.slice(), "iS");
///
/// lexer.next();
/// assert_eq!(lexer.slice(), "UNIMPORtant");
///
/// // We've reached the end of our input, so calling `next` again returns `None`
/// assert_eq!(lexer.next(), None);
/// ```
///
pub trait Logos<'source>: Sized {
    /// The "extras" type, used to add state to a lexer.
    ///
    /// Occasionally you'd like to add extra information to a lexer; perhaps for the sake of debugging, or just to
    /// handle more complicated input. Logos allows you to insert this extra information into a lexer using a concept of
    /// "extras" - a value stored in the lexer that you can access within callbacks and modify as you wish.
    ///
    /// By default, Logos will just use the `()` type for extras. If you'd like to use a different type, you can use the
    /// derive macro's `extras` option.
    type Extras;

    /// The source type that tokens are lexed from.
    ///
    /// This is `str` by default, but will be `[u8]` if you use a non-unicode byte string (or character class) within a
    /// `#[token(...)]` or `#[regex(...)]` definition.
    ///
    /// You can use your own source type by passing a type implementing the [Source] trait to the derive macro's `source`
    /// option.
    type Source: Source + ?Sized + 'source;

    /// The type used to report errors during lexing.
    ///
    /// This is [UnknownToken] by default, but you can use your own
    /// error type by passing a type implementing the [Error] trait to the derive macro's `error` option.
    type Error: Error<'source, Self>;

    /// The heart of Logos.
    ///
    /// This method is called during the lexing process, and is implemented by the `logos-derive` crate. As a reminder,
    /// you should **never implement this trait yourself**. Use the derive macro!
    fn lex(lexer: &mut Lexer<'source, Self>);

    /// Create a new [Lexer] for this token type.
    fn lexer(source: &'source Self::Source) -> Lexer<'source, Self>
    where
        Self::Extras: Default,
    {
        Lexer::new(source)
    }

    /// Create a new [Lexer] for this token type, using the provided `extras` value.
    ///
    /// # Note
    ///
    /// In most cases, you can use [Logos::lexer] instead. You should only use this function if you need to set up your
    /// lexer in a way that doesn't play nicely with the [Default] trait.
    fn lexer_with_extras(
        source: &'source Self::Source,
        extras: Self::Extras,
    ) -> Lexer<'source, Self> {
        Lexer::with_extras(source, extras)
    }
}

/// Used within callbacks to instruct the lexer to skip a token match.
///
/// This type mostly serves as a convenient shorthand. See also [logos::skip][crate::skip] for a predefined callback that returns
/// values of this type.
///
/// # Example
///
/// ```rust
/// use logos::{Logos, Skip};
///
/// #[derive(Logos, Debug, PartialEq)]
/// enum Token<'a> {
///     // This variant matches on "abc" and whitespace, but is ignored because the callback returns `Skip`.
///     // This callback is equivalent to `logos::skip`.
///     #[regex(" |abc", |_| Skip)]
///     Trivia,
///
///     #[regex("[a-zA-Z]+")]
///     Text(&'a str),
/// }
///
/// let tokens: Vec<_> = Token::lexer("Hello abc world").collect();
///
/// assert_eq!(
///     tokens,
///     &[
///         Ok(Token::Text("Hello")),
///         // `abc` is skipped
///         Ok(Token::Text("world")),
///     ],
/// );
/// ```
pub struct Skip;

/// A type that can be used within callbacks to either produce a field for a token or skip a token match.
///
/// # Example
///
/// ```rust
/// use logos::{Logos, Filter};
///
/// #[derive(Logos, Debug, PartialEq)]
/// enum Token {
///     #[regex(r"[ \n\f\t]+", logos::skip)]
///     Whitespace,
///
///     #[regex("[0-9]+", |lex| {
///         let n: u64 = lex.slice().parse().unwrap();
///
///         // Only emit a token if `n` is an even number
///         match n % 2 {
///             0 => Filter::Accept(n),
///             _ => Filter::Skip,
///         }
///     })]
///     EvenNumber(u64)
/// }
///
/// let tokens: Vec<_> = Token::lexer("20 11 42 23 100 8002").collect();
///
/// assert_eq!(
///     tokens,
///     &[
///         Ok(Token::EvenNumber(20)),
///         // 11 isn't an even number, so it's skipped
///         Ok(Token::EvenNumber(42)),
///         // 23 isn't an even number, so it's skipped
///         Ok(Token::EvenNumber(100)),
///         Ok(Token::EvenNumber(8002)),
///     ]
/// );
/// ```
pub enum Filter<C> {
    /// Construct and emit a variant containing a value of type `C`.
    Accept(C),
    /// Skip this token match.
    Skip,
}

/// A predefined callback that unconditionally skips a token match.
///
/// When lexing, you often run into situations where you simply *do not care* about certain parts of your input. Notable
/// cases include things like comments and whitespace in a C-like language - they're not useful later on when parsing,
/// so you have no need to keep them.
///
/// This function serves as an easy shorthand for situations like those, and uses the [Skip] type under the hood. See
/// the [documentation on callbacks](./callback/index.html) for more information.
///
/// # Example
///
/// ```rust
/// use logos::Logos;
///
/// #[derive(Logos, Debug, PartialEq)]
/// enum Token<'a> {
///     // We'll treat "abc" as trivia, and ignore it.
///     #[regex(" |abc", logos::skip)]
///     Trivia,
///
///     #[regex("[a-zA-Z]+")]
///     Text(&'a str),
/// }
///
/// let tokens: Vec<_> = Token::lexer("Hello abc world").collect();
///
/// assert_eq!(
///     tokens,
///     &[
///         Ok(Token::Text("Hello")),
///         // `abc` is ignored
///         Ok(Token::Text("world")),
///     ],
/// );
/// ```
#[inline(always)]
pub fn skip<'source, Token: Logos<'source>>(_: &mut Lexer<'source, Token>) -> Skip {
    Skip
}

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../../README.md"));
}
