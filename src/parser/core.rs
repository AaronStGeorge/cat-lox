use ast::*;
use lexer::*;

pub struct Parser<'a> {
    tokens: &'a [Token],
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [Token]) -> Parser {
        return Parser {
            tokens: input,
            index: 0,
        };
    }

    pub fn parse(&mut self) -> Result<Expression, &'static str> {
        Ok(self.expression())
    }

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
            Some(t) if *t == Token::Equal || *t == Token::NotEqual => {
                Some(t.clone())
            }
            _ => None,
        } {
            self.advance();
            let right = self.comparison();
            expr = Expression::Binary(
                Box::new(expr),
                Box::new(t),
                Box::new(right),
            );
        }

        expr
    }

    fn comparison(&mut self) -> Expression {
        let mut expr = self.addition();

        while let Some(t) = match self.peek() {
            Some(t)
                if *t == Token::Equal || *t == Token::GreaterEqual ||
                    *t == Token::LessThan ||
                    *t == Token::LessEqual =>
            {
                Some(t.clone())
            }
            _ => None,
        } {
            self.advance();
            let right = self.addition();
            expr = Expression::Binary(
                Box::new(expr),
                Box::new(t),
                Box::new(right),
            );
        }

        expr
    }

    fn addition(&mut self) -> Expression {
        let mut expr = self.multiplication();

        while let Some(t) = match self.peek() {
            Some(t) if *t == Token::Minus || *t == Token::Plus => {
                Some(t.clone())
            }
            _ => None,
        } {
            self.advance();
            let right = self.multiplication();
            expr = Expression::Binary(
                Box::new(expr),
                Box::new(t),
                Box::new(right),
            );
        }

        expr
    }

    fn multiplication(&mut self) -> Expression {
        let mut expr = self.unary();

        while let Some(t) = match self.peek() {
            Some(t) if *t == Token::Slash || *t == Token::Asterisk => {
                Some(t.clone())
            }
            _ => None,
        } {
            self.advance();
            let right = self.unary();
            expr = Expression::Binary(
                Box::new(expr),
                Box::new(t),
                Box::new(right),
            );
        }

        expr
    }

    fn unary(&mut self) -> Expression {
        if let Some(t) = match self.peek() {
            Some(t) if *t == Token::Bang || *t == Token::Minus => {
                Some(t.clone())
            }
            _ => None,
        } {
            self.advance();
            let right = self.unary();
            return Expression::Unary(Box::new(t), Box::new(right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Expression {
        match self.advance() {
            Some(t) => Expression::Literal(Box::new(t.clone())),
            None => unreachable!(),
        }
    }
}
