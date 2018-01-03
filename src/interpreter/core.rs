use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::rc::Rc;
use std::mem;
use std::collections::HashMap;

use ast::*;
use lexer::*;
use super::environment::Environment;

pub struct Interpreter {
    current_environment: Environment,
    global_environment: Environment,
    locals: HashMap<Expression, usize>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let global_environment = Environment::global();
        Interpreter {
            current_environment: Environment::new_from(&global_environment),
            global_environment: global_environment,
            locals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, program: &[Statement]) {
        for s in program {
            match self.visit_statement(s) {
                Ok(()) => (),
                Err(err) => match err {
                    CatBoxReturn::Err(s) => println!("Run Time Error: {}", s),
                    CatBoxReturn::Return(_) => {
                        println!("Return can only be used in function scope dummy")
                    }
                },
            }
        }
    }

    fn execute_block(
        &mut self,
        block: &[Statement],
        mut environment: Environment,
    ) -> Result<(), CatBoxReturn> {
        // Swap out environment with desired environment
        mem::swap(&mut self.current_environment, &mut environment);

        for statement in block {
            match self.visit_statement(statement) {
                Ok(()) => (),
                Err(value) => match value {
                    CatBoxReturn::Err(s) => {
                        return Err(CatBoxReturn::Err(s));
                    }
                    CatBoxReturn::Return(t) => {
                        // Swap back current environment
                        mem::swap(&mut self.current_environment, &mut environment);

                        return Err(CatBoxReturn::Return(t));
                    }
                },
            }
        }

        // Swap back current environment
        mem::swap(&mut self.current_environment, &mut environment);

        Ok(())
    }

    pub fn resolve(&mut self, expr: &Expression, i: usize) {
        self.locals.insert(expr.clone(), i);
    }
}

impl MutVisitor for Interpreter {
    type E = Result<Types, String>;
    type S = Result<(), CatBoxReturn>;

    fn visit_expression(&mut self, e: &Expression) -> Self::E {
        match e {
            &Expression::Assignment(ref token, ref expr) => {
                let value = self.visit_expression(expr)?;
                match self.locals.get(e) {
                    Some(distance) => {
                        self.current_environment
                            .assign_at(*distance, &token, value.clone())?
                    }
                    None => self.global_environment.assign(&token, value.clone())?,
                };
                Ok(value)
            }
            &Expression::Binary(ref l_expr, ref token, ref r_expr) => {
                let right = self.visit_expression(r_expr)?;
                let left = self.visit_expression(l_expr)?;
                match (left, token.clone(), right) {
                    (Types::ReturnString(mut ls), Token::Plus, Types::ReturnString(rs)) => {
                        ls.push_str(&rs);
                        Ok(Types::ReturnString(ls))
                    }
                    (Types::Number(n), Token::Plus, Types::ReturnString(mut s))
                    | (Types::ReturnString(mut s), Token::Plus, Types::Number(n)) => {
                        s.push_str(&format!("{}", n));
                        Ok(Types::ReturnString(s))
                    }
                    (Types::Number(ln), t, Types::Number(rn)) => match t {
                        Token::Plus => Ok(Types::Number(ln + rn)),
                        Token::Minus => Ok(Types::Number(ln - rn)),
                        Token::Asterisk => Ok(Types::Number(ln * rn)),
                        Token::Slash => if rn == 0.0 {
                            Err(String::from("No cabrón, I will not divide by zero!"))
                        } else {
                            Ok(Types::Number(ln / rn))
                        },
                        Token::GreaterThan => Ok(Types::Boolean(ln > rn)),
                        Token::GreaterEqual => Ok(Types::Boolean(ln >= rn)),
                        Token::LessThan => Ok(Types::Boolean(ln < rn)),
                        Token::LessEqual => Ok(Types::Boolean(ln <= rn)),
                        Token::Equal => Ok(Types::Boolean(ln == rn)),
                        Token::NotEqual => Ok(Types::Boolean(ln != rn)),
                        _ => Err(String::from("NO! NO you can't do that! Fuuuuuuck!")),
                    },
                    (Types::Nil, t, Types::Nil) => match t {
                        Token::Equal => Ok(Types::Boolean(true)),
                        Token::NotEqual => Ok(Types::Boolean(false)),
                        _ => Err(String::from("Fuck no you asshole I'm not doing that shit!")),
                    },
                    (Types::Boolean(lb), t, Types::Boolean(rb)) => match t {
                        Token::Equal => Ok(Types::Boolean(lb == rb)),
                        Token::NotEqual => Ok(Types::Boolean(lb != rb)),
                        _ => Err(String::from("¡Chinga tu madre!")),
                    },
                    _ => Err(String::from("NO! NO! NO!")),
                }
            }
            &Expression::Call(ref callee_expr, ref argument_exprs) => {
                let callee = match self.visit_expression(callee_expr)? {
                    Types::Callable(inner) => inner,
                    _ => return Err(String::from("You can't call this shit!")),
                };

                if argument_exprs.len() != callee.arity() {
                    return Err(String::from(format!(
                        "This wants {} arguments and you passed it {}, try again dipshit",
                        callee.arity(),
                        argument_exprs.len()
                    )));
                }

                let mut arguments: Vec<Types> = Vec::new();
                for e in argument_exprs {
                    arguments.push(self.visit_expression(e)?);
                }

                Ok(callee.call(self, arguments)?)
            }
            &Expression::Grouping(ref expr) => self.visit_expression(expr),
            &Expression::Literal(ref token) => match token.clone() {
                Token::Number(i) => Ok(Types::Number(i.into())),
                Token::True => Ok(Types::Boolean(true)),
                Token::False => Ok(Types::Boolean(false)),
                Token::Nil => Ok(Types::Nil),
                Token::LoxString(s) => Ok(Types::ReturnString(s)),
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
            &Expression::Unary(ref token, ref expr) => {
                let right = self.visit_expression(expr)?;
                match (right, token.clone()) {
                    (Types::Number(n), Token::Minus) => Ok(Types::Number(-n)),
                    (Types::Nil, Token::Bang) | (Types::Boolean(false), Token::Bang) => {
                        Ok(Types::Boolean(true))
                    }
                    (_, Token::Bang) => Ok(Types::Boolean(false)),
                    _ => Err(String::from("🖕🖕🖕🖕")),
                }
            }
            &Expression::Variable(ref token) => {
                println!("self.locals.len() {}", self.locals.len());
                match self.locals.get(e) {
                    Some(distance) => match self.current_environment.get_at(*distance, token)? {
                        Some(t) => Ok(t),
                        None => Ok(Types::Nil),
                    },
                    None => match self.global_environment.get(token)? {
                        Some(t) => Ok(t),
                        None => Ok(Types::Nil),
                    },
                }
            }
        }
    }

    fn visit_statement(&mut self, s: &Statement) -> Self::S {
        match s {
            &Statement::Block(ref statements) => {
                let mut environment = Environment::new_node(&self.current_environment);

                self.execute_block(statements, environment)?;
                Ok(())
            }
            &Statement::Expression(ref expr) => {
                self.visit_expression(expr)?;
                Ok(())
            }
            &Statement::FunctionDeclaration(ref name_token, ref parameters, ref body) => {
                let cbox_fn = Function {
                    parameters: parameters.clone(),
                    body: body.clone(),
                    closure: Environment::new_from(&self.current_environment),
                };
                self.current_environment.define(
                    &name_token,
                    Some(Types::Callable(Rc::new(Box::new(cbox_fn)))),
                );
                Ok(())
            }
            &Statement::If(ref conditional, ref then, ref else_option) => {
                if is_truthy(&self.visit_expression(conditional)?) {
                    self.visit_statement(then)?;
                } else {
                    if let &Some(ref else_stmt) = else_option {
                        self.visit_statement(else_stmt)?;
                    }
                }

                Ok(())
            }
            &Statement::Print(ref expr) => {
                let result = self.visit_expression(expr)?;
                println!("{}", result);
                Ok(())
            }
            &Statement::Return(ref expr_option) => Err(CatBoxReturn::Return(match expr_option {
                &Some(ref expr) => self.visit_expression(expr)?,
                &None => Types::Nil,
            })),
            &Statement::VariableDeclaration(ref token, ref initializer) => match initializer {
                &Some(ref e) => {
                    let result = self.visit_expression(e)?;
                    Ok(self.current_environment.define(&token, Some(result)))
                }
                &None => Ok(self.current_environment.define(&token, None)),
            },
            &Statement::While(ref expr, ref stmt) => {
                while is_truthy(&self.visit_expression(expr)?) {
                    self.visit_statement(stmt)?
                }

                Ok(())
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Types {
    Number(f64),
    ReturnString(String),
    Boolean(bool),
    Callable(Rc<Box<Callable>>),
    Nil,
}

pub enum CatBoxReturn {
    Err(String),
    Return(Types),
}

impl From<String> for CatBoxReturn {
    fn from(s: String) -> Self {
        CatBoxReturn::Err(s)
    }
}

impl Display for Types {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &Types::Number(n) => write!(f, "{}", n),
            &Types::Boolean(b) => write!(f, "{}", b),
            &Types::ReturnString(ref s) => write!(f, "\"{}\"", s.to_string()),
            &Types::Nil => write!(f, "nil"),
            &Types::Callable(ref c) => write!(f, "{}", c),
        }
    }
}

pub trait Callable: Debug + Display {
    fn arity(&self) -> usize;
    fn call(&self, &mut Interpreter, Vec<Types>) -> Result<Types, String>;
}

fn is_truthy(expression_return: &Types) -> bool {
    match expression_return {
        &Types::Nil | &Types::Boolean(false) => false,
        _ => true,
    }
}

#[derive(Debug)]
pub struct Function {
    parameters: Vec<Token>,
    body: Vec<Statement>,
    closure: Environment,
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.parameters.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        mut arguments: Vec<Types>,
    ) -> Result<Types, String> {
        let mut environment = Environment::new_node(&self.closure);

        // Define parameters as passed arguments
        for (i, arg) in arguments.drain(..).enumerate() {
            environment.define(&self.parameters[i], Some(arg));
        }

        match interpreter.execute_block(&self.body, environment) {
            Ok(()) => Ok(Types::Nil),
            Err(value) => match value {
                CatBoxReturn::Err(s) => Err(s),
                CatBoxReturn::Return(t) => Ok(t),
            },
        }
    }
}


impl Display for Function {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "user defined function")
    }
}
