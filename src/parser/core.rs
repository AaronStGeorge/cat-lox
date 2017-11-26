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

    // Statements ==================================================================================

    // Helpers ====

    fn block(&self) -> Result<Vec<Statement>, &'static str> {
        let mut statements: Vec<Statement> = Vec::new();
        while match self.peek() {
            Some(&Token::RightBrace) => false,
            _ => true,
        } {
            statements.push(self.declaration()?);
        }

        match self.advance() {
            Some(&Token::RightBrace) => Ok(statements),
            _ => Err("You didn't close your fucking block!"),
        }
    }

    // Parsing methods ====

    fn declaration(&self) -> Result<Statement, &'static str> {
        match self.peek() {
            Some(&Token::Let) => {
                self.advance();
                self.var_declaration()
            }
            Some(&Token::Function) => {
                self.advance();
                self.function_declaration()
            }
            _ => self.statement(),
        }
    }

    fn var_declaration(&self) -> Result<Statement, &'static str> {
        match (self.advance(), self.peek()) {
            (Some(token), Some(&Token::Assign)) => {
                self.advance();
                match (self.expression()?, self.peek()) {
                    (e, Some(&Token::Semicolon)) => {
                        self.advance();
                        Ok(Statement::VariableDeclaration(token.clone(), Some(e)))
                    }
                    _ => Err(
                        "OMG!!! It goes let whatever = some shit; How. Fucking. Hard. Is. That.",
                    ),
                }
            }
            (Some(token), Some(&Token::Semicolon)) => {
                self.advance();
                Ok(Statement::VariableDeclaration(token.clone(), None))
            }
            _ => Err("Are you trying to write let a; and failing? Jesus."),
        }
    }

    fn function_declaration(&self) -> Result<Statement, &'static str> {
        match (self.advance(), self.advance()) {
            (Some(name @ &Token::Ident(_)), Some(&Token::LeftParentheses)) => {
                let mut parameters: Vec<Token> = Vec::new();
                if self.peek() != Some(&Token::RightParentheses) {
                    loop {
                        if parameters.len() >= 8 {
                            return Err("Cannot have more than 8 parameters!");
                        }
                        if let Some(t @ &Token::Ident(_)) = self.advance() {
                            parameters.push(t.clone());
                        } else {
                            return Err("That can't be used as a parameter");
                        }
                        match self.peek() {
                            Some(&Token::Comma) => {
                                self.advance();
                            }
                            _ => break,
                        }
                    }
                }
                if self.advance() != Some(&Token::RightParentheses) {
                    return Err("You're going to need to close this function");
                }
                if self.advance() != Some(&Token::LeftBrace) {
                    return Err("You're going to need a function body");
                }
                let statements = self.block()?;
                Ok(Statement::FunctionDeclaration(
                    name.clone(),
                    parameters,
                    statements,
                ))
            }
            _ => Err("Yeah you said this was a function but it doesn't look like one."),
        }
    }

    fn statement(&self) -> Result<Statement, &'static str> {
        match self.peek() {
            Some(&Token::For) => {
                self.advance();
                self.for_statement()
            }
            Some(&Token::If) => {
                self.advance();
                self.if_statement()
            }
            Some(&Token::LeftBrace) => {
                self.advance();
                self.block_statement()
            }
            Some(&Token::Print) => {
                self.advance();
                self.print_statement()
            }
            Some(&Token::While) => {
                self.advance();
                self.while_statement()
            }
            _ => self.expr_statement(),
        }
    }

    fn for_statement(&self) -> Result<Statement, &'static str> {
        match self.peek() {
            Some(&Token::LeftParentheses) => {
                self.advance();
            }
            _ => return Err("There should be a left parentheses after a for! Dick."),
        }

        let initializer = match self.peek() {
            Some(&Token::Let) => {
                self.advance();
                Some(self.var_declaration()?)
            }
            Some(&Token::Semicolon) => None,
            _ => Some(self.expr_statement()?),
        };

        let condition = match self.peek() {
            Some(&Token::Semicolon) => {
                self.advance();
                None
            }
            _ => Some(self.expression()?),
        };

        match self.peek() {
            Some(&Token::Semicolon) => {
                self.advance();
            }
            _ => {
                return Err(
                    "Fucking for statements need fucking semicolons after fucking \
                     conditions! Fuck!",
                )
            }
        }

        let increment = match self.peek() {
            Some(&Token::RightParentheses) => None,
            _ => Some(self.expression()?),
        };

        match self.peek() {
            Some(&Token::RightParentheses) => {
                self.advance();
            }
            _ => return Err("Fucking for statements need a left parenthesis after shit! Fuck!"),
        }

        let mut body = self.statement()?;

        if let Some(increment_inner) = increment {
            body = Statement::Block(vec![body, Statement::Expression(increment_inner)]);
        }

        body = match condition {
            Some(condition_inner) => Statement::While(condition_inner, Box::new(body)),
            None => Statement::While(Expression::Literal(Token::True), Box::new(body)),
        };

        if let Some(initializer_inner) = initializer {
            body = Statement::Block(vec![initializer_inner, body]);
        }

        Ok(body)
    }

    fn if_statement(&self) -> Result<Statement, &'static str> {
        // Fail if there isn't a left parentheses
        // TODO: Write a macro for this
        match self.advance() {
            Some(&Token::LeftParentheses) => (),
            _ => return Err("You need a fucking left parentheses dummy."),
        }
        let condition = self.expression()?;
        // Fail if there isn't a right parentheses
        match self.advance() {
            Some(&Token::RightParentheses) => (),
            _ => return Err("Conditionals have to be surrounded with parentheses dummy."),
        }

        let then_branch = self.statement()?;
        match self.peek() {
            Some(&Token::Else) => {
                self.advance();
                let else_branch = self.statement()?;
                Ok(Statement::If(
                    condition,
                    Box::new(then_branch),
                    Some(Box::new(else_branch)),
                ))
            }
            _ => Ok(Statement::If(condition, Box::new(then_branch), None)),
        }
    }

    fn block_statement(&self) -> Result<Statement, &'static str> {
        Ok(Statement::Block(self.block()?))
    }

    fn print_statement(&self) -> Result<Statement, &'static str> {
        let expr = self.expression()?;

        match self.peek() {
            Some(&Token::Semicolon) => {
                self.advance();
                Ok(Statement::Print(expr))
            }
            _ => Err("There should be a fucking semicolon after this print statement!"),
        }
    }

    fn while_statement(&self) -> Result<Statement, &'static str> {
        match self.peek() {
            Some(&Token::LeftParentheses) => {
                self.advance();
            }
            _ => return Err("There should be a left parentheses after the while! Dick."),
        }

        let condition = self.expression()?;

        match self.peek() {
            Some(&Token::RightParentheses) => {
                self.advance();
            }
            _ => return Err("There should be a right parentheses after the while! Dick."),
        }

        let body = self.statement()?;

        Ok(Statement::While(condition, Box::new(body)))
    }

    fn expr_statement(&self) -> Result<Statement, &'static str> {
        let expr = self.expression()?;

        match self.peek() {
            Some(&Token::Semicolon) => {
                self.advance();
                Ok(Statement::Expression(expr))
            }
            _ => Err("There should be a fucking semicolon after this expression!"),
        }
    }


    // Expressions =================================================================================

    fn expression(&self) -> Result<Expression, &'static str> {
        self.assignment()
    }

    fn assignment(&self) -> Result<Expression, &'static str> {
        let expr = self.or()?;

        if let Some(_) = match self.peek() {
            Some(&Token::Assign) => self.advance(),
            _ => None,
        } {
            let value = self.assignment()?;

            match expr {
                Expression::Variable(token) => {
                    return Ok(Expression::Assignment(token, Box::new(value)))
                }
                _ => return Err("Are you trying to assign something? Get it the fuck right!"),
            }
        }

        Ok(expr)
    }

    fn or(&self) -> Result<Expression, &'static str> {
        let mut expr = self.and()?;

        while let Some(t) = match self.peek() {
            Some(&Token::LogicOr) => self.advance(),
            _ => None,
        } {
            let right = self.and()?;
            expr = Expression::Logical(Box::new(expr), t.clone(), Box::new(right));
        }

        Ok(expr)
    }

    fn and(&self) -> Result<Expression, &'static str> {
        let mut expr = self.equality()?;

        while let Some(t) = match self.peek() {
            Some(&Token::LogicAnd) => self.advance(),
            _ => None,
        } {
            let right = self.equality()?;
            expr = Expression::Logical(Box::new(expr), t.clone(), Box::new(right));
        }

        Ok(expr)
    }

    fn equality(&self) -> Result<Expression, &'static str> {
        let mut expr = self.comparison()?;

        while let Some(t) = match self.peek() {
            Some(&Token::Equal) | Some(&Token::NotEqual) => self.advance(),
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
            Some(&Token::Equal) |
            Some(&Token::GreaterThan) |
            Some(&Token::GreaterEqual) |
            Some(&Token::LessThan) |
            Some(&Token::LessEqual) => self.advance(),
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
            Some(&Token::Minus) | Some(&Token::Plus) => self.advance(),
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
            Some(&Token::Slash) | Some(&Token::Asterisk) => self.advance(),
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
            _ => self.call(),
        }
    }

    fn call(&self) -> Result<Expression, &'static str> {
        let mut expr = self.primary()?;

        while self.peek() == Some(&Token::LeftParentheses) {
            self.advance();
            let mut args: Vec<Expression> = Vec::new();
            if self.peek() != Some(&Token::RightParentheses) {
                loop {
                    if args.len() >= 8 {
                        return Err("More than 8 arguments are not allowed!");
                    }
                    args.push(self.expression()?);
                    match self.peek() {
                        Some(&Token::Comma) => {
                            self.advance();
                        }
                        _ => break,
                    }
                }
            }
            match self.advance() {
                Some(&Token::RightParentheses) => expr = Expression::Call(Box::new(expr), args),
                _ => return Err("If you're trying to fucking call that try harder."),
            }
        }

        Ok(expr)
    }

    fn primary(&self) -> Result<Expression, &'static str> {
        if let Some(t) = self.advance() {
            match *t {
                Token::LeftParentheses => {
                    let expr = self.expression()?;
                    match self.advance() {
                        Some(&Token::RightParentheses) => Ok(Expression::Grouping(Box::new(expr))),
                        _ => Err("There should be a fucking right parentheses here!"),
                    }
                }
                Token::Number(_) |
                Token::Nil |
                Token::True |
                Token::LoxString(_) |
                Token::False => Ok(Expression::Literal(t.clone())),
                Token::Ident(_) => Ok(Expression::Variable(t.clone())),
                _ => Err("What the fuck is this shit!"),
            }
        } else {
            Err("There should be some shit here!")
        }
    }
}
