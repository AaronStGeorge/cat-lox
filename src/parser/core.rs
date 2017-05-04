use ast::*;
use lexer::*;

pub struct Parser {
    lexer: Lexer,

    currToken: Option<Token>,
    peekToken: Option<Token>,
}

// TODO: this should use the same method names as the lexer
impl Parser {
    pub fn new(input: &str) -> Parser {
        let mut parser = Parser {
            lexer: Lexer::new(input),

            currToken: None,
            peekToken: None,
        };

        parser.advance();
        parser.advance();

        parser
    }

    // This should probably just be done with indexes
    fn advance(&mut self) {
        self.currToken = self.peekToken.clone();
        self.peekToken = self.lexer.next();
    }

    fn parseProgram() -> Option<Program> {
        None
    }
}
