use lexer::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

pub trait Expression: Node {
    fn expression_node(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self) -> String;
}

pub struct Program {
    statements: Vec<Box<Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].statement_node()
        } else {
            "".to_string()
        }
    }
}

pub struct Identifier {
    identifier_token: Token,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        format!("{:?}", self.identifier_token)
    }
}

impl Expression for Identifier {
    fn expression_node(&self) -> String {
        format!("{:?}", self.identifier_token)
    }
}

// TODO: I assume this will need to be generic to get sizing
pub struct LetStatement {
    // TODO: look into phantom types to get this properly typed, or just wait
    // until specialization is implemented.
    let_token: Token,
    identifier: Identifier,
    value: Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        format!("{:?}", self.let_token)
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) -> String {
        format!("{:?}", self.let_token)
    }
}
