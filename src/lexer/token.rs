extern crate ordered_float;

use self::ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Token {
    // Characters
    Illegal,
    // Identifiers + literals
    Ident(String),
    LoxString(String),
    Number(OrderedFloat<f64>),
    Nil,
    // Operators
    Assign,
    Asterisk,
    Bang,
    Minus,
    Plus,
    Slash,
    // Logic Operators
    LogicAnd,
    LogicOr,
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
    Else,
    EOF,
    False,
    For,
    Function,
    If,
    Let,
    Print,
    Return,
    True,
    While,
}

pub fn keyword(s: &str) -> Option<Token> {
    match s {
        "and" => Some(Token::LogicAnd),
        "else" => Some(Token::Else),
        "false" => Some(Token::False),
        "fn" => Some(Token::Function),
        "for" => Some(Token::For),
        "if" => Some(Token::If),
        "let" => Some(Token::Let),
        "nil" => Some(Token::Nil),
        "or" => Some(Token::LogicOr),
        "print" => Some(Token::Print),
        "return" => Some(Token::Return),
        "true" => Some(Token::True),
        "while" => Some(Token::While),
        &_ => None,
    }
}
