use lexer::Token;

/// Expression types.
pub enum Expression {
    Binary(Box<Expression>, Token, Box<Expression>),
    Grouping(Box<Expression>),
    Literal(Token),
    Unary(Token, Box<Expression>),
    Variable(String),
}

pub enum Statement {
    Print(Expression),
    Expression(Expression),
    VariableDeclaration(String, Expression),
}
