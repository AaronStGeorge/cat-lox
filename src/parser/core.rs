use ast::*;
use lexer::*;

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(input: &str) -> Parser {
        return Parser {
            tokens: Lexer::new(input).collect(),
            index: 0,
        };
    }

    // This should probably just be done with indexes
    fn advance(&mut self) -> Option<&Token> {
        if self.index >= self.tokens.len() {
            None
        } else {
            self.index += 1;
            Some(&self.tokens[self.index - 1])
        }
    }

    fn peek(&self) -> Option<&Token> {
        if self.index >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[self.index])
        }
    }

    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();

        while let Some(t) = match self.peek() {
            Some(t) if *t == Token::Equal || *t == Token::NotEqual => Some(t.clone()),
            _ => None,
        } {
            self.advance();
            let right = self.comparison();
            expr = Expression::Binary(Box::new(expr), Box::new(t), Box::new(right));
        }

        expr
    }

    fn comparison(&mut self) -> Expression {
        Expression::Literal("1".to_string())
    }
}
