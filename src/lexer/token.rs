#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Characters
    Illegal,
    // Identifiers + literals
    Ident { literal: String },
    Number(f64),
    Nil,
    // Operators
    Assign,
    Asterisk,
    Bang,
    Minus,
    Plus,
    Slash,
    // Order
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
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
    If,
    Else,
    Return,
    True,
    False,
}

pub fn keyword(s: &str) -> Option<Token> {
    match s {
        "fn" => Some(Token::Function),
        "let" => Some(Token::Let),
        "if" => Some(Token::If),
        "else" => Some(Token::Else),
        "return" => Some(Token::Return),
        "true" => Some(Token::True),
        "false" => Some(Token::False),
        "nil" => Some(Token::Nil),
        &_ => None,
    }
}
