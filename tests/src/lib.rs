use logos::source::Source;
use logos::Logos;

use std::fmt;
use std::ops::Range;

mod binary;

type TestCase<'a, Token> = (
    Result<Token, <Token as Logos<'a>>::Error>,
    &'a <<Token as Logos<'a>>::Source as Source>::Slice,
    Range<usize>,
);

#[track_caller]
pub fn assert_lex<'a, Token>(source: &'a Token::Source, tokens: &[TestCase<'a, Token>])
where
    Token: Logos<'a> + fmt::Debug + PartialEq,
    Token::Error: fmt::Debug + PartialEq,
    Token::Extras: Default,
{
    let mut lex = Token::lexer(source);

    for tuple in tokens {
        assert_eq!(
            &(lex.next().expect("Unexpected end"), lex.slice(), lex.span()),
            tuple
        );
    }

    assert_eq!(lex.next(), None);
}
