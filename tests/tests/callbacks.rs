use logos::{Lexer, Logos};
use tests::assert_lex;

mod data {
    use super::*;

    #[derive(Logos, Debug, PartialEq)]
    enum Token<'a> {
        #[regex(r"[ \t\n\f]+", logos::skip)]
        Whitespace,

        #[regex(r"[a-zA-Z]+", |lex| lex.slice())]
        Text(&'a str),

        #[regex(r"-?[0-9]+", |lex| lex.slice().parse().ok())]
        Integer(i64),

        #[regex(r"-?[0-9]+\.[0-9]+", |lex| lex.slice().parse().ok())]
        Float(f64),
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn numbers() {
        let tokens: Vec<_> = Token::lexer("Hello 1 42 -100 pi 3.14 -77.77").collect();

        assert_eq!(
            tokens,
            &[
                Ok(Token::Text("Hello")),
                Ok(Token::Integer(1)),
                Ok(Token::Integer(42)),
                Ok(Token::Integer(-100)),
                Ok(Token::Text("pi")),
                Ok(Token::Float(3.14)),
                Ok(Token::Float(-77.77)),
            ]
        );
    }
}

mod nested_lifetime {
    use super::*;
    use std::borrow::Cow;

    #[derive(Logos, Debug, PartialEq)]
    enum Token<'a> {
        #[regex(r"[ \t\n\f]+", logos::skip)]
        Whitespace,

        #[regex(r"[0-9]+", |lex| {
            let slice = lex.slice();

            slice.parse::<u64>().map(|n| {
                (slice, n)
            }).ok()
        })]
        Integer((&'a str, u64)),

        #[regex(r"[a-z]+", |lex| Cow::Borrowed(lex.slice()))]
        Text(Cow<'a, str>),
    }

    #[test]
    fn supplement_lifetime_in_types() {
        let tokens: Vec<_> = Token::lexer("123 hello 42").collect();

        assert_eq!(
            tokens,
            &[
                Ok(Token::Integer(("123", 123))),
                Ok(Token::Text(Cow::Borrowed("hello"))),
                Ok(Token::Integer(("42", 42))),
            ],
        );
    }
}

mod rust {
    use logos::UnknownToken;

    use super::*;

    /// Adaptation of implementation by matklad:
    /// https://github.com/matklad/fall/blob/527ab331f82b8394949041bab668742868c0c282/lang/rust/syntax/src/rust.fall#L1294-L1324
    fn parse_raw_string(lexer: &mut Lexer<Token>) -> bool {
        // Who needs more then 25 hashes anyway? :)
        let q_hashes = concat!('"', "######", "######", "######", "######", "######");
        let closing = &q_hashes[..lexer.slice().len() - 1]; // skip initial 'r'

        lexer
            .remainder()
            .find(closing)
            .map(|i| lexer.bump(i + closing.len()))
            .is_some()
    }

    #[derive(Logos, Debug, Clone, Copy, PartialEq)]
    enum Token {
        #[regex(r"[ \t\n\f]+", logos::skip)]
        Whitespace,

        #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
        Ident,

        #[regex("r#*\"", parse_raw_string)]
        RawString,
    }

    #[test]
    fn raw_strings() {
        assert_lex(
            " r\"foo\" r#\"bar\"# r#####\"baz\"##### r###\"error\"## ",
            &[
                (Ok(Token::RawString), "r\"foo\"", 1..7),
                (Ok(Token::RawString), "r#\"bar\"#", 8..16),
                (Ok(Token::RawString), "r#####\"baz\"#####", 17..33),
                (Err(UnknownToken), "r###\"", 34..39),
                (Ok(Token::Ident), "error", 39..44),
                (Err(UnknownToken), "\"", 44..45),
                (Err(UnknownToken), "#", 45..46),
                (Err(UnknownToken), "#", 46..47),
            ],
        );
    }
}
