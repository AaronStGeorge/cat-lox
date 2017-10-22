use ast::*;
use lexer::*;
use std::fmt;
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

    pub fn interpret(&mut self, program: &[Statement]) -> Result<(), String> {
        for s in program {
            self.visit_statement(s)?;
        }

        Ok(())
    }
}

impl MutVisitor for Interpreter {
    type E = Result<ExpressionReturn, String>;
    type S = Result<(), String>;

    fn visit_expression(&mut self, e: &Expression) -> Self::E {
        match e {
            &Expression::Assignment(ref token, ref expr) => unimplemented!(),
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
                        Token::GreaterEqual => Ok(ExpressionReturn::Boolean(ln > rn)),
                        Token::LessThan => Ok(ExpressionReturn::Boolean(ln > rn)),
                        Token::LessEqual => Ok(ExpressionReturn::Boolean(ln > rn)),
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
            &Expression::Grouping(ref e) => self.visit_expression(e),
            &Expression::Literal(ref t) => match t.clone() {
                Token::Number(i) => Ok(ExpressionReturn::Number(i)),
                Token::True => Ok(ExpressionReturn::Boolean(true)),
                Token::False => Ok(ExpressionReturn::Boolean(false)),
                Token::Nil => Ok(ExpressionReturn::Nil),
                Token::LoxString(s) => Ok(ExpressionReturn::ReturnString(s)),
                _ => Err(String::from("🐑💨")),
            },
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
            &Expression::Variable(ref s) => match self.environment.get(s)? {
                Some(e) => Ok(e),
                None => Ok(ExpressionReturn::Nil),
            },
        }
    }

    fn visit_statement(&mut self, s: &Statement) -> Self::S {
        match s {
            &Statement::Print(ref e) => {
                let result = self.visit_expression(e)?;
                println!("{}", result);
                Ok(())
            }
            &Statement::Expression(ref e) => {
                self.visit_expression(e)?;
                Ok(())
            }
            &Statement::VariableDeclaration(ref token, ref initializer) => match initializer {
                &Some(ref e) => {
                    let result = self.visit_expression(e)?;
                    Ok(self.environment.define(token, Some(result)))
                }
                &None => Ok(self.environment.define(token, None)),
            },
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

impl fmt::Display for ExpressionReturn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ExpressionReturn::Number(n) => write!(f, "{}", n),
            &ExpressionReturn::Boolean(b) => write!(f, "{}", b),
            &ExpressionReturn::ReturnString(ref s) => write!(f, "\"{}\"", s.to_string()),
            &ExpressionReturn::Nil => write!(f, "nil"),
        }
    }
}
