#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Characters
    Illegal,
    // Identifiers + literals
    Ident(String),
    LoxString(String),
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
    Else,
    False,
    Function,
    If,
    Let,
    Print,
    Return,
    True,
    EOF,
}

pub fn keyword(s: &str) -> Option<Token> {
    match s {
        "else" => Some(Token::Else),
        "false" => Some(Token::False),
        "fn" => Some(Token::Function),
        "if" => Some(Token::If),
        "let" => Some(Token::Let),
        "nil" => Some(Token::Nil),
        "print" => Some(Token::Print),
        "return" => Some(Token::Return),
        "true" => Some(Token::True),
        &_ => None,
    }
}
