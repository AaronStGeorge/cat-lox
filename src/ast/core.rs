use lexer::Token;

/// Expression types.
pub enum Expression {
    Assignment(Token, Box<Expression>),
    Binary(Box<Expression>, Token, Box<Expression>),
    Grouping(Box<Expression>),
    Literal(Token),
    Unary(Token, Box<Expression>),
    Variable(Token),
}

pub enum Statement {
    Print(Expression),
    Expression(Expression),
    VariableDeclaration(String, Option<Expression>),
}
