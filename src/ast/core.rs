extern crate uuid;

use lexer::Token;
use std::hash::{Hash, Hasher};

use self::uuid::Uuid;

#[derive(Clone, Debug)]
pub enum Expression {
    Assignment {
        id: Uuid,
        name: Token,
        expr: Box<Expression>,
    },
    Binary {
        id: Uuid,
        l_expr: Box<Expression>,
        operator: Token,
        r_expr: Box<Expression>,
    },
    Call {
        id: Uuid,
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Get {
        id: Uuid,
        object: Box<Expression>,
        name: Token,
    },
    Grouping {
        id: Uuid,
        expr: Box<Expression>,
    },
    Literal {
        id: Uuid,
        token: Token,
    },
    Logical {
        id: Uuid,
        l_expr: Box<Expression>,
        operator: Token,
        r_expr: Box<Expression>,
    },
    Set {
        id: Uuid,
        name: Token,
        object: Box<Expression>,
        value: Box<Expression>,
    },
    Super {
        id: Uuid,
        method: Token,
    },
    This {
        id: Uuid,
    },
    Unary {
        id: Uuid,
        operator: Token,
        expr: Box<Expression>,
    },
    Variable {
        id: Uuid,
        name: Token,
    },
}

impl Expression {
    fn get_id(&self) -> &Uuid {
        match self {
            &Expression::Assignment { ref id, .. } => id,
            &Expression::Binary { ref id, .. } => id,
            &Expression::Call { ref id, .. } => id,
            &Expression::Get { ref id, .. } => id,
            &Expression::Grouping { ref id, .. } => id,
            &Expression::Literal { ref id, .. } => id,
            &Expression::Logical { ref id, .. } => id,
            &Expression::Set { ref id, .. } => id,
            &Expression::Super { ref id, .. } => id,
            &Expression::This { ref id, .. } => id,
            &Expression::Unary { ref id, .. } => id,
            &Expression::Variable { ref id, .. } => id,
        }
    }
}

impl Hash for Expression {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let id = self.get_id();
        id.hash(state)
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Expression) -> bool {
        let self_id = self.get_id();
        let other_id = other.get_id();
        self_id == other_id
    }
}

impl Eq for Expression {}

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
