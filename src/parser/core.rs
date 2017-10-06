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

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements: Vec<Statement> = vec![];
        let mut errs: Vec<&'static str> = vec![];
        while let Some(_) = self.peek() {
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(err) => {
                    errs.push(err);
                    self.synchronize();
                }
            }
        }

        if !errs.is_empty() {
            Err(errs.join("\n"))
        } else {
            Ok(statements)
        }
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

    fn previous(&self) -> Option<&Token> {
        let index = self.index.get();
        if index - 1 >= self.tokens.len() || index == 0 {
            None
        } else {
            Some(&self.tokens[index - 1])
        }
    }

    fn synchronize(&self) -> () {
        self.advance();

        while let Some(next_token) = self.peek() {
            if let Some(previous_token) = self.previous() {
                if *previous_token == Token::Semicolon {
                    return;
                }
            }

            match next_token {
                &Token::Function | &Token::Print | &Token::If | &Token::Return | &Token::Let => {
                    return
                }
                _ => (),
            }

            self.advance();
        }
    }

    fn declaration(&self) -> Result<Statement, &'static str> {
        match self.peek() {
            Some(&Token::Let) => self.var_declaration(),
            _ => self.statement(),
        }
    }

    fn statement(&self) -> Result<Statement, &'static str> {
        match self.peek() {
            Some(&Token::Print) => {
                self.advance();
                self.print_statement()
            }
            _ => self.expr_statement(),
        }
    }

    fn print_statement(&self) -> Result<Statement, &'static str> {
        let expr = self.expression()?;

        match self.peek() {
            Some(t) if *t == Token::Semicolon => {
                self.advance();
                Ok(Statement::Print(expr))
            }
            _ => Err("There should be a fucking semicolon after this print statement!"),
        }
    }

    fn expr_statement(&self) -> Result<Statement, &'static str> {
        let expr = self.expression()?;

        match self.peek() {
            Some(t) if *t == Token::Semicolon => {
                self.advance();
                Ok(Statement::Expression(expr))
            }
            _ => Err("There should be a fucking semicolon after this expression!"),
        }
    }

    fn var_declaration(&self) -> Result<Statement, &'static str> {
        let var_name = match (self.advance(), self.advance(), self.advance()) {
            (Some(&Token::Let), Some(&Token::Ident(ref n)), Some(&Token::Assign)) => {
                Some(n.clone())
            }
            _ => None,
        };

        match (var_name, self.expression()?, self.peek()) {
            (Some(s), e, Some(&Token::Semicolon)) => {
                self.advance();
                Ok(Statement::VariableDeclaration(s, e))
            }
            _ => Err("OMG!!! It goes let whatever = some shit; How. Fucking. Hard. Is. That. "),
        }
    }

    fn expression(&self) -> Result<Expression, &'static str> {
        self.equality()
    }

    fn equality(&self) -> Result<Expression, &'static str> {
        let mut expr = self.comparison()?;

        while let Some(t) = match self.peek() {
            Some(t) if *t == Token::Equal || *t == Token::NotEqual => self.advance(),
            _ => None,
        } {
            let right = self.comparison()?;
            expr = Expression::Binary(Box::new(expr), t.clone(), Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&self) -> Result<Expression, &'static str> {
        let mut expr = self.addition()?;

        while let Some(t) = match self.peek() {
            Some(t)
                if *t == Token::Equal || *t == Token::GreaterThan || *t == Token::GreaterEqual
                    || *t == Token::LessThan || *t == Token::LessEqual =>
            {
                self.advance()
            }
            _ => None,
        } {
            let right = self.addition()?;
            expr = Expression::Binary(Box::new(expr), t.clone(), Box::new(right));
        }

        Ok(expr)
    }

    fn addition(&self) -> Result<Expression, &'static str> {
        let mut expr = self.multiplication()?;

        while let Some(t) = match self.peek() {
            Some(t) if *t == Token::Minus || *t == Token::Plus => self.advance(),
            _ => None,
        } {
            let right = self.multiplication()?;
            expr = Expression::Binary(Box::new(expr), t.clone(), Box::new(right));
        }

        Ok(expr)
    }

    fn multiplication(&self) -> Result<Expression, &'static str> {
        let mut expr = self.unary()?;

        while let Some(t) = match self.peek() {
            Some(t) if *t == Token::Slash || *t == Token::Asterisk => self.advance(),
            _ => None,
        } {
            let right = self.unary()?;
            expr = Expression::Binary(Box::new(expr), t.clone(), Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&self) -> Result<Expression, &'static str> {
        match self.peek() {
            Some(t) if *t == Token::Bang || *t == Token::Minus => {
                self.advance();
                let right = self.unary()?;
                return Ok(Expression::Unary(t.clone(), Box::new(right)));
            }
            _ => self.primary(),
        }
    }

    fn primary(&self) -> Result<Expression, &'static str> {
        if let Some(t) = self.advance() {
            match *t {
                Token::LeftParentheses => {
                    let expr = self.expression()?;
                    match self.advance() {
                        Some(t) if *t == Token::RightParentheses => {
                            Ok(Expression::Grouping(Box::new(expr)))
                        }
                        _ => Err("There should be a fucking right parentheses here!"),
                    }
                }
                Token::Number(_) |
                Token::Ident(_) |
                Token::Nil |
                Token::True |
                Token::LoxString(_) |
                Token::False => Ok(Expression::Literal(t.clone())),
                _ => Err("What the fuck is this shit!"),
            }
        } else {
            Err("There should be some shit here!")
        }
    }
}
