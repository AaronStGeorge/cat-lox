#[derive(Debug, PartialEq)]
pub enum Token {
    // Characters
    Illegal,
    // Identifiers + literals
    Ident { literal: String },
    Int { literal: String },
    // Operators
    Assign,
    Plus,
    // Delimiters
    Comma,
    Semicolon,
    // Parens
    LParen,
    RParen,
    // Braces
    LBrace,
    RBrace,
    // Keywords
    Function,
    Let,
}


pub fn control_token(c: &char) -> Option<Token> {
    match *c {
        '+' => Some(Token::Plus),
        ';' => Some(Token::Semicolon),
        '=' => Some(Token::Assign),
        ',' => Some(Token::Comma),
        ')' => Some(Token::RParen),
        '(' => Some(Token::LParen),
        '}' => Some(Token::RBrace),
        '{' => Some(Token::LBrace),
        _ => None,
    }
}

pub fn keyword(s: &str) -> Option<Token> {
    match s {
        "let" => Some(Token::Let),
        "fn" => Some(Token::Function),
        &_ => None,
    }
}

pub fn is_whitespace(c: &char) -> bool {
    *c == ' ' || *c == '\n' || *c == '\t' || *c == '\r'
}