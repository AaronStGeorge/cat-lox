use lexer::Token;

#[derive(Clone, Debug)]
pub enum Expression {
    Assignment {
        id: usize,
        name: Token,
        expr: Box<Expression>,
    },
    Binary {
        id: usize,
        l_expr: Box<Expression>,
        operator: Token,
        r_expr: Box<Expression>,
    },
    Call {
        id: usize,
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Get {
        id: usize,
        object: Box<Expression>,
        name: Token,
    },
    Grouping {
        id: usize,
        expr: Box<Expression>,
    },
    Literal {
        id: usize,
        token: Token,
    },
    Logical {
        id: usize,
        l_expr: Box<Expression>,
        operator: Token,
        r_expr: Box<Expression>,
    },
    Set {
        id: usize,
        name: Token,
        object: Box<Expression>,
        value: Box<Expression>,
    },
    Super {
        id: usize,
        method: Token,
    },
    This {
        id: usize,
    },
    Unary {
        id: usize,
        operator: Token,
        expr: Box<Expression>,
    },
    Variable {
        id: usize,
        name: Token,
    },
}

impl Expression {
    pub fn get_id(&self) -> usize {
        match self {
            &Expression::Assignment { id, .. } => id,
            &Expression::Binary { id, .. } => id,
            &Expression::Call { id, .. } => id,
            &Expression::Get { id, .. } => id,
            &Expression::Grouping { id, .. } => id,
            &Expression::Literal { id, .. } => id,
            &Expression::Logical { id, .. } => id,
            &Expression::Set { id, .. } => id,
            &Expression::Super { id, .. } => id,
            &Expression::This { id, .. } => id,
            &Expression::Unary { id, .. } => id,
            &Expression::Variable { id, .. } => id,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    Class {
        name: Token,
        super_class: Option<Expression>,
        methods: Vec<Statement>,
    },
    Block(Vec<Statement>),
    Expression(Expression),
    FunctionDeclaration(Token, Vec<Token>, Vec<Statement>),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    Return(Option<Expression>),
    VariableDeclaration(Token, Option<Expression>),
    While(Expression, Box<Statement>),
}
