use lexer::Token;

/// Expression types.
pub enum Expression {
    Literal(Token),
    Unary(Token, Box<Expression>),
    Binary(Box<Expression>, Token, Box<Expression>),
    Grouping(Box<Expression>),
}

pub enum Statement {
    Print(Expression),
    Expression(Expression),
}
