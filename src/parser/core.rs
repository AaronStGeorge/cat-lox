use ast::*;
use lexer::*;
use std::cell::Cell;

pub struct Parser<'a> {
    tokens: &'a [Token],
    index: Cell<usize>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [Token]) -> Parser {
        return Parser {
            tokens: input,
            index: Cell::new(0),
        };
    }

    pub fn parse(&mut self) -> Result<Expression, &'static str> {
        self.expression()
    }

    fn advance(&self) -> Option<&Token> {
        let previous_index = self.index.get();
        if previous_index >= self.tokens.len() {
            None
        } else {
            self.index.set(previous_index + 1);
            Some(&self.tokens[previous_index])
        }
    }

    fn peek(&self) -> Option<&Token> {
        let index = self.index.get();
        if index >= self.tokens.len() {
            None
        } else {
            Some(&self.tokens[index])
        }
    }

    fn expression(&self) -> Result<Expression, &'static str> {
        self.equality()
    }

    fn equality(&self) -> Result<Expression, &'static str> {
        let mut expr = self.comparison()?;

        while let Some(t) = match self.peek() {
            Some(t) if *t == Token::Equal || *t == Token::NotEqual => {
                self.advance()
            }
            _ => None,
        } {
            let right = self.comparison()?;
            expr = Expression::Binary(
                Box::new(expr),
                Box::new(t.clone()),
                Box::new(right),
            );
        }

        Ok(expr)
    }

    fn comparison(&self) -> Result<Expression, &'static str> {
        let mut expr = self.addition()?;

        while let Some(t) = match self.peek() {
            Some(t)
                if *t == Token::Equal || *t == Token::GreaterEqual ||
                    *t == Token::LessThan ||
                    *t == Token::LessEqual =>
            {
                self.advance()
            }
            _ => None,
        } {
            let right = self.addition()?;
            expr = Expression::Binary(
                Box::new(expr),
                Box::new(t.clone()),
                Box::new(right),
            );
        }

        Ok(expr)
    }

    fn addition(&self) -> Result<Expression, &'static str> {
        let mut expr = self.multiplication()?;

        while let Some(t) = match self.peek() {
            Some(t) if *t == Token::Minus || *t == Token::Plus => {
                self.advance()
            }
            _ => None,
        } {
            let right = self.multiplication()?;
            expr = Expression::Binary(
                Box::new(expr),
                Box::new(t.clone()),
                Box::new(right),
            );
        }

        Ok(expr)
    }

    fn multiplication(&self) -> Result<Expression, &'static str> {
        let mut expr = self.unary()?;

        while let Some(t) = match self.peek() {
            Some(t) if *t == Token::Slash || *t == Token::Asterisk => {
                self.advance()
            }
            _ => None,
        } {
            let right = self.unary()?;
            expr = Expression::Binary(
                Box::new(expr),
                Box::new(t.clone()),
                Box::new(right),
            );
        }

        Ok(expr)
    }

    fn unary(&self) -> Result<Expression, &'static str> {
        match self.peek() {
            Some(t) if *t == Token::Bang || *t == Token::Minus => {
                self.advance();
                let right = self.unary()?;
                return Ok(
                    Expression::Unary(Box::new(t.clone()), Box::new(right)),
                );
            }
            _ => self.primary(),
        }
    }

    fn primary(&self) -> Result<Expression, &'static str> {
        match self.advance() {
            Some(t) if *t == Token::LeftParentheses => {
                let expr = self.expression()?;

                // Same grossness if self.advance didn't need to take a
                // mutable reference this could be a lot nicer.
                if let Some(_) = match self.advance() {
                    Some(t) if *t == Token::RightParentheses => Some(t.clone()),
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
            Some(t) => Ok(Expression::Literal(Box::new(t.clone()))),
            None => Err("There should be some shit here!"),
        }
    }
}
