use ast::*;
use lexer::*;

pub struct Parser {
    lexer: Lexer,

    curr_token: Option<Token>,
    peek_token: Option<Token>,
}

// TODO: this should use the same method names as the lexer
impl Parser {
    pub fn new(input: &str) -> Parser {
        let mut parser = Parser {
            lexer: Lexer::new(input),

            curr_token: None,
            peek_token: None,
        };

        parser.advance();
        parser.advance();

        parser
    }

    // This should probably just be done with indexes
    fn advance(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next();
    }

    fn parse_program() -> Option<Program> {
        None
    }
}
