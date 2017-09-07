use lexer::Token;

/// Expression types.
pub enum Expression {
    Literal(String), // TODO: Literal should maybe be an ADT rather than a string.
    Unary(Box<Token>, Box<Expression>),
    Binary(Box<Expression>, Box<Token>, Box<Expression>),
    Grouping(Box<Expression>),
}
