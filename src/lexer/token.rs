#[derive(Debug, PartialEq)]
pub enum Token {
    // Characters
    Illegal,
    // Identifiers + literals
    Ident { literal: String },
    Int { literal: String },
    // Operators
    Assign,
    Asterisk,
    Bang,
    Minus,
    Plus,
    Slash,
    // Order
    LessThan,
    GreaterThan,
    // Equality
    Equal,
    NotEqual,
    // Delimiters
    Comma,
    Semicolon,
    // Parentheses
    LeftParentheses,
    RightParentheses,
    // Braces
    LeftBrace,
    RightBrace,
    // Keywords
    Function,
    Let,
}

pub fn keyword(s: &str) -> Option<Token> {
    match s {
        "let" => Some(Token::Let),
        "fn" => Some(Token::Function),
        &_ => None,
    }
}