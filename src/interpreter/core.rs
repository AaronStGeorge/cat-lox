use std;

use ast::*;
use lexer::*;
use super::environment::Environment;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(),
        }
    }

    // TODO: this is here because in the future it might make sense to have the
    // repl take expressions again, if that is never implemented this should be removed.
    pub fn evaluate(&mut self, e: &Expression) -> Result<ExpressionReturn, String> {
        self.visit_expression(e)
    }

    pub fn interpret<W: std::io::Write>(&mut self, program: &[Statement], w: &mut W) {
        for s in program {
            match self.visit_statement(s, w) {
                Ok(()) => (),
                Err(err) => {
                    writeln!(w, "Run Time Error: {}", err).unwrap();
                }
            }
        }
    }
}

impl MutVisitor for Interpreter {
    type E = Result<ExpressionReturn, String>;
    type S = Result<(), String>;

    fn visit_expression(&mut self, e: &Expression) -> Self::E {
        match e {
            &Expression::Assignment(ref token, ref expr) => {
                let value = self.visit_expression(expr)?;
                self.environment.assign(&token, value.clone())?;
                Ok(value)
            }
            &Expression::Binary(ref l_expr, ref token, ref r_expr) => {
                let right = self.visit_expression(r_expr)?;
                let left = self.visit_expression(l_expr)?;
                match (left, token.clone(), right) {
                    (
                        ExpressionReturn::ReturnString(mut ls),
                        Token::Plus,
                        ExpressionReturn::ReturnString(rs),
                    ) => {
                        ls.push_str(&rs);
                        Ok(ExpressionReturn::ReturnString(ls))
                    }
                    (ExpressionReturn::Number(ln), t, ExpressionReturn::Number(rn)) => match t {
                        Token::Plus => Ok(ExpressionReturn::Number(ln + rn)),
                        Token::Minus => Ok(ExpressionReturn::Number(ln - rn)),
                        Token::Asterisk => Ok(ExpressionReturn::Number(ln * rn)),
                        Token::Slash => if rn == 0.0 {
                            Err(String::from("No cabrón, I will not divide by zero!"))
                        } else {
                            Ok(ExpressionReturn::Number(ln / rn))
                        },
                        Token::GreaterThan => Ok(ExpressionReturn::Boolean(ln > rn)),
                        Token::GreaterEqual => Ok(ExpressionReturn::Boolean(ln >= rn)),
                        Token::LessThan => Ok(ExpressionReturn::Boolean(ln < rn)),
                        Token::LessEqual => Ok(ExpressionReturn::Boolean(ln <= rn)),
                        Token::Equal => Ok(ExpressionReturn::Boolean(ln == rn)),
                        Token::NotEqual => Ok(ExpressionReturn::Boolean(ln != rn)),
                        _ => Err(String::from("NO! NO you can't do that! Fuuuuuuck!")),
                    },
                    (ExpressionReturn::Nil, t, ExpressionReturn::Nil) => match t {
                        Token::Equal => Ok(ExpressionReturn::Boolean(true)),
                        Token::NotEqual => Ok(ExpressionReturn::Boolean(false)),
                        _ => Err(String::from("Fuck no you asshole I'm not doing that shit!")),
                    },
                    (ExpressionReturn::Boolean(lb), t, ExpressionReturn::Boolean(rb)) => match t {
                        Token::Equal => Ok(ExpressionReturn::Boolean(lb == rb)),
                        Token::NotEqual => Ok(ExpressionReturn::Boolean(lb != rb)),
                        _ => Err(String::from("¡Chinga tu madre!")),
                    },
                    _ => Err(String::from("NO! NO! NO!")),
                }
            }
            &Expression::Call(ref e, ref t, ref args) => unimplemented!(),
            &Expression::Grouping(ref e) => self.visit_expression(e),
            &Expression::Literal(ref t) => match t.clone() {
                Token::Number(i) => Ok(ExpressionReturn::Number(i)),
                Token::True => Ok(ExpressionReturn::Boolean(true)),
                Token::False => Ok(ExpressionReturn::Boolean(false)),
                Token::Nil => Ok(ExpressionReturn::Nil),
                Token::LoxString(s) => Ok(ExpressionReturn::ReturnString(s)),
                _ => Err(String::from("🐑💨")),
            },
            &Expression::Logical(ref l_expr, ref token, ref r_expr) => {
                let left_result = self.visit_expression(l_expr)?;

                if token == &Token::LogicOr {
                    if is_truthy(&left_result) {
                        return Ok(left_result);
                    }
                } else {
                    if !is_truthy(&left_result) {
                        return Ok(left_result);
                    }
                }

                self.visit_expression(r_expr)
            }
            &Expression::Unary(ref t, ref e) => {
                let right = self.visit_expression(e)?;
                match (right, t.clone()) {
                    (ExpressionReturn::Number(n), Token::Minus) => Ok(ExpressionReturn::Number(-n)),
                    (ExpressionReturn::Nil, Token::Bang) |
                    (ExpressionReturn::Boolean(false), Token::Bang) => {
                        Ok(ExpressionReturn::Boolean(true))
                    }
                    (_, Token::Bang) => Ok(ExpressionReturn::Boolean(false)),
                    _ => Err(String::from("🖕🖕🖕🖕")),
                }
            }
            &Expression::Variable(ref token) => match self.environment.get(token)? {
                Some(e) => Ok(e),
                None => Ok(ExpressionReturn::Nil),
            },
        }
    }

    fn visit_statement<W: std::io::Write>(&mut self, s: &Statement, w: &mut W) -> Self::S {
        match s {
            &Statement::Block(ref statements) => {
                self.environment.open();

                for statement in statements {
                    self.visit_statement(statement, w)?;
                }

                self.environment.close()?;
                Ok(())
            }
            &Statement::Expression(ref e) => {
                self.visit_expression(e)?;
                Ok(())
            }
            &Statement::If(ref conditional, ref then_stmt, ref else_option) => {
                if is_truthy(&self.visit_expression(conditional)?) {
                    self.visit_statement(then_stmt, w)?;
                } else {
                    if let &Some(ref else_stmt) = else_option {
                        self.visit_statement(else_stmt, w)?;
                    }
                }

                Ok(())
            }
            &Statement::Print(ref expr) => {
                let result = self.visit_expression(expr)?;
                writeln!(w, "{}", result).unwrap();
                Ok(())
            }
            &Statement::VariableDeclaration(ref token, ref initializer) => match initializer {
                &Some(ref e) => {
                    let result = self.visit_expression(e)?;
                    Ok(self.environment.define(&token, Some(result)))
                }
                &None => Ok(self.environment.define(&token, None)),
            },
            &Statement::While(ref expr, ref stmt) => {
                while is_truthy(&self.visit_expression(expr)?) {
                    self.visit_statement(stmt, w)?
                }

                Ok(())
            }
        }
    }
}

// This seems like the types the language support. This should probably just be Types.
#[derive(Debug, Clone)]
pub enum ExpressionReturn {
    Number(f64),
    ReturnString(String),
    Boolean(bool),
    Nil,
}

impl std::fmt::Display for ExpressionReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &ExpressionReturn::Number(n) => write!(f, "{}", n),
            &ExpressionReturn::Boolean(b) => write!(f, "{}", b),
            &ExpressionReturn::ReturnString(ref s) => write!(f, "\"{}\"", s.to_string()),
            &ExpressionReturn::Nil => write!(f, "nil"),
        }
    }
}

fn is_truthy(expression_return: &ExpressionReturn) -> bool {
    match expression_return {
        &ExpressionReturn::Nil | &ExpressionReturn::Boolean(false) => false,
        _ => true,
    }
}
