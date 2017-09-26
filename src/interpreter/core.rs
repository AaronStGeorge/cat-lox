use ast::*;
use lexer::*;

pub struct Interpreter<'a> {
    expr: &'a Expression,
}

impl<'a> Interpreter<'a> {
    pub fn new(expression: &'a Expression) -> Interpreter {
        Interpreter { expr: expression }
    }

    pub fn interpret(&self) -> Result<String, &'static str> {
        match self.visit_expression(self.expr)? {
            ExpressionReturn::Number(r) => Ok(format!("{}", r)),
            ExpressionReturn::Boolean(r) => Ok(format!("{}", r)),
            ExpressionReturn::ReturnString(s) => Ok(format!("\"{}\"", s)),
            ExpressionReturn::Nil => Ok("nil".to_string()),
        }
    }
}

impl<'a> Visitor<Result<ExpressionReturn, &'static str>> for Interpreter<'a> {
    fn visit_expression(&self, e: &Expression) -> Result<ExpressionReturn, &'static str> {
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
