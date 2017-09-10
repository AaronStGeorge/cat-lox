use lexer::Token;

/// Expression types.
pub enum Expression {
    Literal(Box<Token>),
    Unary(Box<Token>, Box<Expression>),
    Binary(Box<Expression>, Box<Token>, Box<Expression>),
    Grouping(Box<Expression>),
}
