use lexer::token::*;

pub struct Lexer {
    input: Vec<char>,
    index: usize,
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match self.advance() {
            None => None,
            // Operators
            Some('+') => Some(Token::Plus),
            Some('*') => Some(Token::Asterisk),
            Some('<') => Some(Token::LessThan),
            Some('>') => Some(Token::GreaterThan),
            Some('(') => Some(Token::LeftParentheses),
            Some(')') => Some(Token::RightParentheses),
            Some(',') => Some(Token::Comma),
            Some(';') => Some(Token::Semicolon),
            Some('{') => Some(Token::LeftBrace),
            Some('}') => Some(Token::RightBrace),
            Some('=') => {
                match self.peek() {
                    Some('=') => {
                        self.advance();
                        Some(Token::Equal)
                    }
                    _ => Some(Token::Assign),
                }
            }
            Some('/') => {
                match self.peek() {
                    // comments
                    Some('/') => {
                        self.advance();
                        while let Some(current_char) = self.advance() {
                            if current_char == '\n' {
                                break;
                            }
                        }
                        self.next()
                    }
                    _ => Some(Token::Slash),
                }
            }
            // Whitespace (must be checked after comments)
            Some(' ') => self.next(),
            Some('\t') => self.next(),
            Some('\r') => self.next(),
            Some('\n') => self.next(),
            // literal, keyword, or int
            Some(current_char) => {
                let mut literal = String::new();
                literal.push(current_char);

                loop {
                    match self.peek() {
                        Some(next) => {
                            if is_state_change_char(&next) {
                                break;
                            }
                        }
                        None => break,
                    }

                    if let Some(current_char) = self.advance() {
                        literal.push(current_char);
                    }
                }


                // return keyword or literal
                if keyword(&literal).is_some() {
                    keyword(&literal)
                } else if literal.chars().all(|c| c.is_digit(10)) {
                    Some(Token::Int { literal: literal })
                } else {
                    Some(Token::Ident { literal: literal })
                }

            }
        }
    }
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            index: 0,
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.index >= self.input.len() {
            None
        } else {
            self.index += 1;
            Some(self.input[self.index - 1])
        }
    }

    fn peek(&self) -> Option<char> {
        if self.index >= self.input.len() {
            None
        } else {
            Some(self.input[self.index])
        }
    }
}

/// A char that indicates that state is changing
///
/// TODO: if we ever need to add a new state, both this and the next
/// function above need to be changed. That violates the open closed
/// principle, investigate refactoring.
fn is_state_change_char(c: &char) -> bool {
    let blacklist = vec![' ', '\n', '+', ';', '=', ',', ')', '(', '}', '{'];
    blacklist.contains(c)
}