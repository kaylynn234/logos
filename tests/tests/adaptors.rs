use logos::Logos;
use std::{cell::Cell, rc::Rc};

#[derive(Debug, PartialEq)]
pub struct Extras {
    important_number: usize,
    cool_numbers: Vec<usize>,
    very_important_number_do_not_lose: Rc<Cell<usize>>,
}

impl Drop for Extras {
    fn drop(&mut self) {
        let important = self.very_important_number_do_not_lose.get();
        self.very_important_number_do_not_lose.set(important + 1);
    }
}

#[derive(Logos, Debug, PartialEq)]
#[logos(extras = Extras)]
pub enum SideEffectfulToken {
    #[token("a")]
    A,
    #[token("b")]
    B,
    #[token("c")]
    C,
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"\s+", logos::skip)]
    Trivia,
    #[token("alpha")]
    Alpha,
    #[token("beta")]
    Beta,
    #[token("gamma")]
    Gamma,

    Scary,
}

mod adaptors {
    use super::*;
    use logos::LexerExt;

    #[test]
    fn boxed() {
        let must_not_lose = Rc::new(Cell::default());

        let extras = Extras {
            important_number: 12,
            cool_numbers: vec![10, 34, 5, 9, 18, 2, 4500, 7729],
            very_important_number_do_not_lose: Rc::clone(&must_not_lose),
        };

        let mut boxed = SideEffectfulToken::lexer_with_extras("abca", extras).boxed();

        boxed.next();
        boxed.next();
        boxed.next();

        assert_eq!(boxed.slice(), "c");
        assert_eq!(boxed.span(), 2..3);

        let original = boxed.into_lexer();

        let new_extras = Extras {
            important_number: 12,
            cool_numbers: vec![10, 34, 5, 9, 18, 2, 4500, 7729],
            // This should not have been dropped yet, so it should still be 0.
            very_important_number_do_not_lose: Rc::new(Cell::default()),
        };

        assert_eq!(original.extras, new_extras);

        drop(original);

        assert_eq!(must_not_lose.get(), 1);
    }

    #[test]
    fn peeked() {
        let mut lexer = Token::lexer("alpha   beta gamma").spanned().lookahead();

        assert_eq!(lexer.peek().unwrap(), &Ok((Token::Alpha, 0..5)));
        assert_eq!(lexer.peek().unwrap(), &Ok((Token::Alpha, 0..5)));

        assert_eq!(lexer.next().unwrap(), Ok((Token::Alpha, 0..5)));

        let (token, _) = lexer.peek_mut().unwrap().as_mut().unwrap();

        *token = Token::Scary;

        assert_eq!(lexer.next().unwrap(), Ok((Token::Scary, 8..12)));

        let next = lexer
            .next_if(|result| result.as_ref().unwrap().0 == Token::Gamma)
            .unwrap();

        assert_eq!(next, Ok((Token::Gamma, 13..18)));

        assert!(lexer.next().is_none());
    }

    #[test]
    fn mapped() {
        let mut fooble = 0;

        let mut lexer = Token::lexer("alpha alpha alpha alpha").map_with_lexer(|result, _| {
            result.map(|token| {
                fooble += 1;
                (token, fooble)
            })
        });

        assert_eq!(lexer.next().unwrap(), Ok((Token::Alpha, 1)));
        assert_eq!(lexer.next().unwrap(), Ok((Token::Alpha, 2)));
        assert_eq!(lexer.next().unwrap(), Ok((Token::Alpha, 3)));
        assert_eq!(lexer.next().unwrap(), Ok((Token::Alpha, 4)));
        // We've reached the end of input, so this should be `None`.
        assert!(lexer.next().is_none());
    }
}
