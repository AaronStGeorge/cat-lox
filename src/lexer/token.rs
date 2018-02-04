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
    Dot,
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
    Class,
    Else,
    EOF,
    False,
    For,
    Function,
    If,
    Let,
    Print,
    Return,
    This,
    True,
    While,
}

pub fn keyword(s: &str) -> Option<Token> {
    match s {
        "and" => Some(Token::LogicAnd),
        "class" => Some(Token::Class),
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
        "this" => Some(Token::This),
        "true" => Some(Token::True),
        "while" => Some(Token::While),
        &_ => None,
    }
}
