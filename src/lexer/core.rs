use super::token::*;

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
            Some('-') => Some(Token::Minus),
            Some('*') => Some(Token::Asterisk),
            Some('(') => Some(Token::LeftParentheses),
            Some(')') => Some(Token::RightParentheses),
            Some(',') => Some(Token::Comma),
            Some(';') => Some(Token::Semicolon),
            Some('{') => Some(Token::LeftBrace),
            Some('}') => Some(Token::RightBrace),
            Some('<') => match self.peek() {
                Some('=') => {
                    self.advance();
                    Some(Token::LessEqual)
                }
                _ => Some(Token::LessThan),
            },
            Some('>') => match self.peek() {
                Some('=') => {
                    self.advance();
                    Some(Token::GreaterEqual)
                }
                _ => Some(Token::GreaterThan),
            },
            Some('=') => match self.peek() {
                Some('=') => {
                    self.advance();
                    Some(Token::Equal)
                }
                _ => Some(Token::Assign),
            },
            Some('!') => match self.peek() {
                Some('=') => {
                    self.advance();
                    Some(Token::NotEqual)
                }
                _ => Some(Token::Bang),
            },
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
            Some('"') => {
                let mut literal = String::new();
                while let Some(current_char) = self.advance() {
                    if current_char == '"' {
                        break;
                    }
                    literal.push(current_char);
                }
                Some(Token::LoxString(literal))
            }
            // Whitespace (must be checked after comments)
            Some(' ') => self.next(),
            Some('\t') => self.next(),
            Some('\r') => self.next(),
            Some('\n') => self.next(),
            // literal, keyword, or number
            Some(current_char) => {
                // Todo: maybe it would be preferable to store a reference to a
                // slice rather than storing a new heap allocated string.
                let mut literal = String::new();
                literal.push(current_char);

                loop {
                    match self.peek() {
                        Some(next) => if is_blacklisted(&next) {
                            break;
                        },
                        None => break,
                    }

                    if let Some(current_char) = self.advance() {
                        literal.push(current_char);
                    }
                }

                if keyword(&literal).is_some() {
                    keyword(&literal)
                } else if literal.chars().all(|c| c.is_digit(10) || c == '.') {
                    Some(Token::Number(literal.parse::<f64>().unwrap()))
                } else {
                    Some(Token::Ident(literal))
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

/// Is this char allowed to be in a literal?
///
/// TODO: if we ever need to add a new state, both this and the next
/// function above need to be changed. That violates the open closed
/// principle, investigate refactoring.
fn is_blacklisted(c: &char) -> bool {
    let blacklist = vec![
        '+',
        '-',
        '*',
        '<',
        '>',
        '(',
        ')',
        ',',
        ';',
        '{',
        '}',
        '=',
        '!',
        '/',
        ' ',
        '\t',
        '\r',
        '\n',
    ];
    blacklist.contains(c)
}
