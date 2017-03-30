use std::str::Chars;
use std::iter::Peekable;

use lexer::token::*;

// TOOD: investigate moving to a more official state machine
// https://hoverbear.org/2016/10/12/rust-state-machine-pattern/


pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match self.input.next() {
            Some(c) => {
                // whitespace
                if is_whitespace(&c) {
                    return self.next();
                }

                // control token
                if let Some(token) = control_token(&c) {
                    return Some(token);
                }

                // literal or keyword
                let mut literal = String::new();
                literal.push(c);

                loop {
                    match self.input.peek() {
                        Some(next) => {
                            if control_token(&next).is_some() || is_whitespace(&next) {
                                break;
                            }
                        }
                        None => break,
                    }

                    if let Some(c) = self.input.next() {
                        literal.push(c);
                    }
                }


                // return keyword or litteral
                if keyword(&literal).is_some() {
                    keyword(&literal)
                } else if literal.chars().all(|c| c.is_digit(10)) {
                    Some(Token::Int { literal: literal })
                } else {
                    Some(Token::Ident { literal: literal })
                }
            }
            None => None,
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer { input: input.chars().peekable() }
    }
}