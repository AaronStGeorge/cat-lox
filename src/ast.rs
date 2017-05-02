use lexer::Token;

trait Node {
    fn token_literal(&self) -> String;
}

trait Expression : Node {
    fn expression_node(&self) -> String;
}

trait Statement : Node {
    fn statement_node(&self) -> String;
}

struct Program {
    statements : Vec<Box<Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String{
        if self.statements.len() > 0 {
            self.statements[0].statement_node()
        } else {
            "".to_string()
        }
    }
}

struct Identifier {
    identifier_token: Token,
    value: String
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        "let".to_string()
    }
}

struct LetStatement {
    // TODO: look into phantom types to get this properly typed, or just wait
    // until specialization is implemented.
    let_token : Token,
    identifier : Token, 
    value : Expression,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        "let".to_string()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) -> String {
        "let".to_string()
    }
}