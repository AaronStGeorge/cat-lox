use lexer::Token;

#[derive(Clone, Debug)]
pub enum Expression {
    Assignment(Token, Box<Expression>),
    Binary(Box<Expression>, Token, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
    Grouping(Box<Expression>),
    Literal(Token),
    Logical(Box<Expression>, Token, Box<Expression>),
    Unary(Token, Box<Expression>),
    Variable(Token),
}

#[derive(Clone, Debug)]
pub enum Statement {
    Block(Vec<Statement>),
    Expression(Expression),
    FunctionDeclaration(Token, Vec<Token>, Vec<Statement>),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    Print(Expression),
    Return(Option<Expression>),
    VariableDeclaration(Token, Option<Expression>),
    While(Expression, Box<Statement>),
}
