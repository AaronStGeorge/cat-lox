use ast::*;
use lexer::*;
use std::fmt;

pub struct Interpreter<'a> {
    program: &'a [Statement],
}

impl<'a> Interpreter<'a> {
    pub fn new(statements: &'a [Statement]) -> Interpreter {
        Interpreter {
            program: statements,
        }
    }

    // TODO: this is here because in the future it might make sense to have the
    // repl take expressions again, if that is never implemented this should be removed.
    pub fn evaluate(&self, e: &Expression) -> Result<ExpressionReturn, &'static str> {
        self.visit_expression(e)
    }

    pub fn interpret(&self) -> Result<(), &'static str> {
        for s in self.program {
            self.visit_statement(s)?;
        }

        Ok(())
    }
}

impl<'a> Visitor for Interpreter<'a> {
    type E = Result<ExpressionReturn, &'static str>;
    type S = Result<(), &'static str>;

    fn visit_expression(&self, e: &Expression) -> Self::E {
        match e {
            &Expression::Literal(ref t) => match t.clone() {
                Token::Number(i) => Ok(ExpressionReturn::Number(i)),
                Token::True => Ok(ExpressionReturn::Boolean(true)),
                Token::False => Ok(ExpressionReturn::Boolean(false)),
                Token::Nil => Ok(ExpressionReturn::Nil),
                Token::LoxString(s) => Ok(ExpressionReturn::ReturnString(s)),
                _ => Err("🐑💨"),
            },
            &Expression::Grouping(ref e) => self.visit_expression(e),
            &Expression::Unary(ref t, ref e) => {
                let right = self.visit_expression(e)?;
                match (right, t.clone()) {
                    (ExpressionReturn::Number(n), Token::Minus) => Ok(ExpressionReturn::Number(-n)),
                    (ExpressionReturn::Nil, Token::Bang) |
                    (ExpressionReturn::Boolean(false), Token::Bang) => {
                        Ok(ExpressionReturn::Boolean(true))
                    }
                    (_, Token::Bang) => Ok(ExpressionReturn::Boolean(false)),
                    _ => Err("🖕🖕🖕🖕"),
                }
            }
            &Expression::Binary(ref l, ref t, ref r) => {
                let right = self.visit_expression(r)?;
                let left = self.visit_expression(l)?;
                match (left, t.clone(), right) {
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
                            Err("No cabrón, I will not divide by zero!")
                        } else {
                            Ok(ExpressionReturn::Number(ln / rn))
                        },
                        Token::GreaterThan => Ok(ExpressionReturn::Boolean(ln > rn)),
                        Token::GreaterEqual => Ok(ExpressionReturn::Boolean(ln > rn)),
                        Token::LessThan => Ok(ExpressionReturn::Boolean(ln > rn)),
                        Token::LessEqual => Ok(ExpressionReturn::Boolean(ln > rn)),
                        Token::Equal => Ok(ExpressionReturn::Boolean(ln == rn)),
                        Token::NotEqual => Ok(ExpressionReturn::Boolean(ln != rn)),
                        _ => Err("NO! NO you can't do that! Fuuuuuuck!"),
                    },
                    (ExpressionReturn::Nil, t, ExpressionReturn::Nil) => match t {
                        Token::Equal => Ok(ExpressionReturn::Boolean(true)),
                        Token::NotEqual => Ok(ExpressionReturn::Boolean(false)),
                        _ => Err("Fuck no you asshole I'm not doing that shit!"),
                    },
                    (ExpressionReturn::Boolean(lb), t, ExpressionReturn::Boolean(rb)) => match t {
                        Token::Equal => Ok(ExpressionReturn::Boolean(lb == rb)),
                        Token::NotEqual => Ok(ExpressionReturn::Boolean(lb != rb)),
                        _ => Err("¡Chinga tu madre!"),
                    },
                    _ => Err("NO! NO! NO!"),
                }
            }
        }
    }

    fn visit_statement(&self, s: &Statement) -> Self::S {
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
            _ => unimplemented!(),
        }
    }
}

// This type feels a little silly. Maybe this could just be the expression
// literal branch of the tree?
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
            &ExpressionReturn::ReturnString(ref s) => write!(f, "{}", s.to_string()),
            &ExpressionReturn::Nil => write!(f, "nil"),
        }
    }
}
