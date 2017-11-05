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
    pub fn evaluate(&mut self, e: &Expression) -> Result<CatBoxTypes, String> {
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
    type E = Result<CatBoxTypes, String>;
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
                        CatBoxTypes::ReturnString(mut ls),
                        Token::Plus,
                        CatBoxTypes::ReturnString(rs),
                    ) => {
                        ls.push_str(&rs);
                        Ok(CatBoxTypes::ReturnString(ls))
                    }
                    (CatBoxTypes::Number(ln), t, CatBoxTypes::Number(rn)) => match t {
                        Token::Plus => Ok(CatBoxTypes::Number(ln + rn)),
                        Token::Minus => Ok(CatBoxTypes::Number(ln - rn)),
                        Token::Asterisk => Ok(CatBoxTypes::Number(ln * rn)),
                        Token::Slash => if rn == 0.0 {
                            Err(String::from("No cabrón, I will not divide by zero!"))
                        } else {
                            Ok(CatBoxTypes::Number(ln / rn))
                        },
                        Token::GreaterThan => Ok(CatBoxTypes::Boolean(ln > rn)),
                        Token::GreaterEqual => Ok(CatBoxTypes::Boolean(ln >= rn)),
                        Token::LessThan => Ok(CatBoxTypes::Boolean(ln < rn)),
                        Token::LessEqual => Ok(CatBoxTypes::Boolean(ln <= rn)),
                        Token::Equal => Ok(CatBoxTypes::Boolean(ln == rn)),
                        Token::NotEqual => Ok(CatBoxTypes::Boolean(ln != rn)),
                        _ => Err(String::from("NO! NO you can't do that! Fuuuuuuck!")),
                    },
                    (CatBoxTypes::Nil, t, CatBoxTypes::Nil) => match t {
                        Token::Equal => Ok(CatBoxTypes::Boolean(true)),
                        Token::NotEqual => Ok(CatBoxTypes::Boolean(false)),
                        _ => Err(String::from("Fuck no you asshole I'm not doing that shit!")),
                    },
                    (CatBoxTypes::Boolean(lb), t, CatBoxTypes::Boolean(rb)) => match t {
                        Token::Equal => Ok(CatBoxTypes::Boolean(lb == rb)),
                        Token::NotEqual => Ok(CatBoxTypes::Boolean(lb != rb)),
                        _ => Err(String::from("¡Chinga tu madre!")),
                    },
                    _ => Err(String::from("NO! NO! NO!")),
                }
            }
            &Expression::Call(ref callee_expr, ref t, ref args) => {
                let callee = self.evaluate(callee_expr);

                unimplemented!()
            }
            &Expression::Grouping(ref expr) => self.visit_expression(expr),
            &Expression::Literal(ref t) => match t.clone() {
                Token::Number(i) => Ok(CatBoxTypes::Number(i)),
                Token::True => Ok(CatBoxTypes::Boolean(true)),
                Token::False => Ok(CatBoxTypes::Boolean(false)),
                Token::Nil => Ok(CatBoxTypes::Nil),
                Token::LoxString(s) => Ok(CatBoxTypes::ReturnString(s)),
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
                    (CatBoxTypes::Number(n), Token::Minus) => Ok(CatBoxTypes::Number(-n)),
                    (CatBoxTypes::Nil, Token::Bang) |
                    (CatBoxTypes::Boolean(false), Token::Bang) => Ok(CatBoxTypes::Boolean(true)),
                    (_, Token::Bang) => Ok(CatBoxTypes::Boolean(false)),
                    _ => Err(String::from("🖕🖕🖕🖕")),
                }
            }
            &Expression::Variable(ref token) => match self.environment.get(token)? {
                Some(e) => Ok(e),
                None => Ok(CatBoxTypes::Nil),
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
pub enum CatBoxTypes {
    Number(f64),
    ReturnString(String),
    Boolean(bool),
    Nil,
}

impl std::fmt::Display for CatBoxTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &CatBoxTypes::Number(n) => write!(f, "{}", n),
            &CatBoxTypes::Boolean(b) => write!(f, "{}", b),
            &CatBoxTypes::ReturnString(ref s) => write!(f, "\"{}\"", s.to_string()),
            &CatBoxTypes::Nil => write!(f, "nil"),
        }
    }
}

trait Calllable {
    fn cal(&self, &mut Interpreter, &[CatBoxTypes]);
}

fn is_truthy(expression_return: &CatBoxTypes) -> bool {
    match expression_return {
        &CatBoxTypes::Nil | &CatBoxTypes::Boolean(false) => false,
        _ => true,
    }
}
