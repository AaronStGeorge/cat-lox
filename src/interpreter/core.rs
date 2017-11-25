use std::fmt::{Display, Formatter};
use std::fmt;
use std::io::Write;
use std::rc::Rc;

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
    pub fn evaluate(&mut self, e: &Expression) -> Result<CatBoxType, String> {
        self.visit_expression(e)
    }

    pub fn interpret<W: Write>(&mut self, program: &[Statement], w: &mut W) {
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
    type E = Result<CatBoxType, String>;
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
                        CatBoxType::ReturnString(mut ls),
                        Token::Plus,
                        CatBoxType::ReturnString(rs),
                    ) => {
                        ls.push_str(&rs);
                        Ok(CatBoxType::ReturnString(ls))
                    }
                    (CatBoxType::Number(ln), t, CatBoxType::Number(rn)) => match t {
                        Token::Plus => Ok(CatBoxType::Number(ln + rn)),
                        Token::Minus => Ok(CatBoxType::Number(ln - rn)),
                        Token::Asterisk => Ok(CatBoxType::Number(ln * rn)),
                        Token::Slash => if rn == 0.0 {
                            Err(String::from("No cabrón, I will not divide by zero!"))
                        } else {
                            Ok(CatBoxType::Number(ln / rn))
                        },
                        Token::GreaterThan => Ok(CatBoxType::Boolean(ln > rn)),
                        Token::GreaterEqual => Ok(CatBoxType::Boolean(ln >= rn)),
                        Token::LessThan => Ok(CatBoxType::Boolean(ln < rn)),
                        Token::LessEqual => Ok(CatBoxType::Boolean(ln <= rn)),
                        Token::Equal => Ok(CatBoxType::Boolean(ln == rn)),
                        Token::NotEqual => Ok(CatBoxType::Boolean(ln != rn)),
                        _ => Err(String::from("NO! NO you can't do that! Fuuuuuuck!")),
                    },
                    (CatBoxType::Nil, t, CatBoxType::Nil) => match t {
                        Token::Equal => Ok(CatBoxType::Boolean(true)),
                        Token::NotEqual => Ok(CatBoxType::Boolean(false)),
                        _ => Err(String::from("Fuck no you asshole I'm not doing that shit!")),
                    },
                    (CatBoxType::Boolean(lb), t, CatBoxType::Boolean(rb)) => match t {
                        Token::Equal => Ok(CatBoxType::Boolean(lb == rb)),
                        Token::NotEqual => Ok(CatBoxType::Boolean(lb != rb)),
                        _ => Err(String::from("¡Chinga tu madre!")),
                    },
                    _ => Err(String::from("NO! NO! NO!")),
                }
            }
            &Expression::Call(ref callee_expr, ref arg_exprs) => {
                let callee = match self.evaluate(callee_expr)? {
                    CatBoxType::Callable(inner) => inner,
                    _ => return Err(String::from("You can't call this shit!")),
                };

                if arg_exprs.len() != callee.arity() {
                    return Err(String::from(format!(
                        "This wants {} arguments and you passed it {}, try again dipshit",
                        callee.arity(),
                        arg_exprs.len()
                    )));
                }

                let mut arguments: Vec<CatBoxType> = Vec::new();
                for e in arg_exprs {
                    arguments.push(self.evaluate(e)?);
                }

                Ok(callee.call(self, &arguments))
            }
            &Expression::Grouping(ref expr) => self.visit_expression(expr),
            &Expression::Literal(ref t) => match t.clone() {
                Token::Number(i) => Ok(CatBoxType::Number(i)),
                Token::True => Ok(CatBoxType::Boolean(true)),
                Token::False => Ok(CatBoxType::Boolean(false)),
                Token::Nil => Ok(CatBoxType::Nil),
                Token::LoxString(s) => Ok(CatBoxType::ReturnString(s)),
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
                    (CatBoxType::Number(n), Token::Minus) => Ok(CatBoxType::Number(-n)),
                    (CatBoxType::Nil, Token::Bang) | (CatBoxType::Boolean(false), Token::Bang) => {
                        Ok(CatBoxType::Boolean(true))
                    }
                    (_, Token::Bang) => Ok(CatBoxType::Boolean(false)),
                    _ => Err(String::from("🖕🖕🖕🖕")),
                }
            }
            &Expression::Variable(ref token) => match self.environment.get(token)? {
                Some(e) => Ok(e),
                None => Ok(CatBoxType::Nil),
            },
        }
    }

    fn visit_statement<W: Write>(&mut self, s: &Statement, w: &mut W) -> Self::S {
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

#[derive(Clone)]
pub enum CatBoxType {
    Number(f64),
    ReturnString(String),
    Boolean(bool),
    Callable(Rc<Box<Callable>>),
    Nil,
}

impl Display for CatBoxType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &CatBoxType::Number(n) => write!(f, "{}", n),
            &CatBoxType::Boolean(b) => write!(f, "{}", b),
            &CatBoxType::ReturnString(ref s) => write!(f, "\"{}\"", s.to_string()),
            &CatBoxType::Nil => write!(f, "nil"),
            &CatBoxType::Callable(ref c) => write!(f, "{}", c),
        }
    }
}

pub trait Callable: Display {
    fn arity(&self) -> usize;
    fn call(&self, &mut Interpreter, &[CatBoxType]) -> CatBoxType;
}

fn is_truthy(expression_return: &CatBoxType) -> bool {
    match expression_return {
        &CatBoxType::Nil | &CatBoxType::Boolean(false) => false,
        _ => true,
    }
}
