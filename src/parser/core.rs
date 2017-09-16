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
        self.expression()
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

    fn expression(&mut self) -> Result<Expression, &'static str> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, &'static str> {
        let mut expr = self.comparison()?;

        while let Some(t) = match self.peek() {
            Some(t) if *t == Token::Equal || *t == Token::NotEqual => {
                Some(t.clone())
            }
            _ => None,
        } {
            self.advance();
            let right = self.comparison()?;
            expr = Expression::Binary(
                Box::new(expr),
                Box::new(t),
                Box::new(right),
            );
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression, &'static str> {
        let mut expr = self.addition()?;

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
            let right = self.addition()?;
            expr = Expression::Binary(
                Box::new(expr),
                Box::new(t),
                Box::new(right),
            );
        }

        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expression, &'static str> {
        let mut expr = self.multiplication()?;

        while let Some(t) = match self.peek() {
            Some(t) if *t == Token::Minus || *t == Token::Plus => {
                Some(t.clone())
            }
            _ => None,
        } {
            self.advance();
            let right = self.multiplication()?;
            expr = Expression::Binary(
                Box::new(expr),
                Box::new(t),
                Box::new(right),
            );
        }

        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Expression, &'static str> {
        let mut expr = self.unary()?;

        while let Some(t) = match self.peek() {
            Some(t) if *t == Token::Slash || *t == Token::Asterisk => {
                Some(t.clone())
            }
            _ => None,
        } {
            self.advance();
            let right = self.unary()?;
            expr = Expression::Binary(
                Box::new(expr),
                Box::new(t),
                Box::new(right),
            );
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, &'static str> {
        if let Some(t) = match self.peek() {
            Some(t) if *t == Token::Bang || *t == Token::Minus => {
                Some(t.clone())
            }
            _ => None,
        } {
            self.advance();
            let right = self.unary()?;
            return Ok(Expression::Unary(Box::new(t), Box::new(right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, &'static str> {
        // TODO: this is super ugly. It's here to be able to get an owned
        // version of t and prevent a mutable reference to *self being borrowed
        // for the scope of the whole function. Interior mutability primitives
        // like Cell might be able to help here.
        if let Some(t) = match self.advance() {
            Some(t) => Some(t.clone()),
            _ => None,
        } {
            match t {
                Token::LeftParentheses => {
                    let expr = self.expression()?;

                    // Same grossness if self.advance didn't need to take a
                    // mutable reference this could be a lot nicer.
                    if let Some(_) = match self.advance() {
                        Some(t) if *t == Token::RightParentheses => {
                            Some(t.clone())
                        }
                        _ => None,
                    } {
                        Ok(Expression::Grouping(Box::new(expr)))
                    } else {
                        Err("There should be a fucking right parentheses here!")
                    }
                }
                // TODO: this will match all tokens including things that don't
                // make any sense here like ")". This should only accept a
                // portion of what it does.
                _ => Ok(Expression::Literal(Box::new(t.clone()))),
            }
        } else {
            Err("There should be some shit here!")
        }
    }
}
