use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::rc::Rc;
use std::mem;
use std::collections::HashMap;
use std::cell::RefCell;

use ast::*;
use lexer::*;
use super::environment::Environment;

pub struct Interpreter {
    current_environment: Environment,
    global_environment: Environment,
    locals: HashMap<usize, usize>,
}

impl Interpreter {
    pub fn new(output: Box<Fn(&str)>) -> Interpreter {
        let global_environment = Environment::global(output);
        Interpreter {
            current_environment: global_environment.clone(),
            global_environment: global_environment,
            locals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, program: &[Statement]) -> Result<(), String> {
        for s in program {
            match self.visit_statement(s) {
                Ok(()) => (),
                Err(err) => match err {
                    CatBoxReturn::Err(error) => return Err(error),
                    CatBoxReturn::Return(_) => {
                        return Err(String::from(
                            "Return can only be used in function scope dummy",
                        ))
                    }
                },
            }
        }
        Ok(())
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

    fn call_callable(
        &mut self,
        callee: &Callable,
        arguments: &Vec<Expression>,
    ) -> Result<Types, String> {
        if arguments.len() != callee.arity() {
            return Err(String::from(format!(
                "This wants {} arguments and you passed it {}, try again dipshit",
                callee.arity(),
                arguments.len()
            )));
        }

        let mut interpreted_arguments: Vec<Types> = Vec::new();
        for expr in arguments {
            interpreted_arguments.push(self.visit_expression(expr)?);
        }

        Ok(callee.call(self, interpreted_arguments)?)
    }

    pub fn resolve(&mut self, expr: &Expression, i: usize) {
        self.locals.insert(expr.get_id(), i);
    }
}

impl MutVisitor for Interpreter {
    type E = Result<Types, String>;
    type S = Result<(), CatBoxReturn>;

    fn visit_expression(&mut self, e: &Expression) -> Self::E {
        match e {
            &Expression::Assignment {
                ref name, ref expr, ..
            } => {
                let value = self.visit_expression(expr)?;
                match self.locals.get(&e.get_id()) {
                    Some(distance) => {
                        self.current_environment
                            .assign_at(*distance, &name, value.clone())?
                    }
                    None => self.global_environment.assign(&name, value.clone())?,
                };
                Ok(value)
            }
            &Expression::Binary {
                ref l_expr,
                ref operator,
                ref r_expr,
                ..
            } => {
                let right = self.visit_expression(r_expr)?;
                let left = self.visit_expression(l_expr)?;
                match (left, operator.clone(), right) {
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
            &Expression::Call {
                ref callee,
                ref arguments,
                ..
            } => match self.visit_expression(callee)? {
                Types::Callable(inner) => self.call_callable(&(**inner), arguments),
                Types::Class(inner) => self.call_callable(&(*inner), arguments),
                _ => return Err(String::from("You can't call this shit!")),
            },
            &Expression::Get {
                ref name,
                ref object,
                ..
            } => match self.visit_expression(object)? {
                Types::Instance(mut instance) => match name {
                    &Token::Ident(ref name) => match instance.get(name) {
                        Some(get_return) => Ok(get_return),
                        None => Err(format!("{} is a fucking undefined property!", name)),
                    },
                    _ => unreachable!(),
                },
                _ => Err(String::from("Only instances have properties asshole!")),
            },
            &Expression::Grouping { ref expr, .. } => self.visit_expression(expr),
            &Expression::Literal { ref token, .. } => match token.clone() {
                Token::Number(i) => Ok(Types::Number(i.into())),
                Token::True => Ok(Types::Boolean(true)),
                Token::False => Ok(Types::Boolean(false)),
                Token::Nil => Ok(Types::Nil),
                Token::LoxString(s) => Ok(Types::ReturnString(s)),
                _ => Err(String::from("🐑💨")),
            },
            &Expression::Logical {
                ref l_expr,
                ref operator,
                ref r_expr,
                ..
            } => {
                let left_result = self.visit_expression(l_expr)?;

                if operator == &Token::LogicOr {
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
            &Expression::This { .. } => {
                if let Some(distance) = self.locals.get(&e.get_id()) {
                    if let Some(instance) =
                        self.current_environment.get_at(*distance, &Token::This)?
                    {
                        return Ok(instance);
                    }
                }
                Err(String::from(
                    "Internal interpreter error: shit is fucked with this",
                ))
            }
            &Expression::Unary {
                ref operator,
                ref expr,
                ..
            } => {
                let right = self.visit_expression(expr)?;
                match (right, operator.clone()) {
                    (Types::Number(n), Token::Minus) => Ok(Types::Number(-n)),
                    (Types::Nil, Token::Bang) | (Types::Boolean(false), Token::Bang) => {
                        Ok(Types::Boolean(true))
                    }
                    (_, Token::Bang) => Ok(Types::Boolean(false)),
                    _ => Err(String::from("🖕🖕🖕🖕")),
                }
            }
            &Expression::Set {
                ref name,
                ref object,
                ref value,
                ..
            } => match (name, self.visit_expression(object)?) {
                (&Token::Ident(ref name), Types::Instance(ref instance)) => {
                    let value = self.visit_expression(value)?;
                    instance.set(name.clone(), value.clone());
                    Ok(value)
                }
                _ => Err(String::from("Only instances have fields dumbass!")),
            },
            &Expression::Super { ref method, .. } => {
                if let Some(distance) = self.locals.get(&e.get_id()) {
                    if let Some(Types::Class(super_class)) =
                        self.current_environment.get_at(*distance, &Token::Super)?
                    {
                        // "this" is always one level nearer than "super"'s environment.
                        if let Some(Types::Instance(instance)) = self.current_environment
                            .get_at(*distance + 1, &Token::This)?
                        {
                            match method {
                                &Token::Ident(ref method) => {
                                    match super_class.class_data.find_method(method, &instance) {
                                        Some(thing) => {
                                            return Ok(thing);
                                        }
                                        None => {
                                            return Err(format!("Undefined property {}", method));
                                        }
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                }
                Err(String::from(
                    "Internal interpreter error: shit is fucked with self",
                ))
            }
            &Expression::Variable { ref name, .. } => match self.locals.get(&e.get_id()) {
                Some(distance) => match self.current_environment.get_at(*distance, name)? {
                    Some(t) => Ok(t),
                    None => Ok(Types::Nil),
                },
                None => match self.global_environment.get(name)? {
                    Some(t) => Ok(t),
                    None => Ok(Types::Nil),
                },
            },
        }
    }

    fn visit_statement(&mut self, s: &Statement) -> Self::S {
        match s {
            &Statement::Class {
                name: ref class_name,
                ref methods,
                ref super_class,
            } => match class_name {
                &Token::Ident(ref name_string) => {
                    let mut super_environment = Environment::new_node(&self.current_environment);
                    let super_class_data_option = match super_class {
                        &Some(ref expr) => match self.visit_expression(expr)? {
                            Types::Class(class) => {
                                mem::swap(&mut self.current_environment, &mut super_environment);

                                self.current_environment.define(
                                    &Token::Super,
                                    Some(Types::Class(Rc::new(Class {
                                        class_data: class.class_data.clone(),
                                    }))),
                                );

                                Some(class.class_data.clone())
                            }
                            _ => {
                                return Err(CatBoxReturn::Err(String::from(
                                    "Superclass must be a class!",
                                )))
                            }
                        },
                        &None => None,
                    };

                    let mut methods_map = HashMap::new();
                    for method_statement in methods {
                        match method_statement {
                            &Statement::FunctionDeclaration(ref name, ref parameters, ref body) => {
                                let name = match name {
                                    &Token::Ident(ref name) => name.clone(),
                                    _ => unreachable!(),
                                };

                                let method = Function {
                                    parameters: parameters.clone(),
                                    body: body.clone(),
                                    closure: self.current_environment.clone(),
                                };

                                methods_map.insert(name, method);
                            }
                            _ => unreachable!(),
                        }
                    }

                    if super_class_data_option.is_some() {
                        mem::swap(&mut self.current_environment, &mut super_environment);
                    }

                    let class_data = ClassData {
                        name: name_string.clone(),
                        methods: methods_map,
                        super_class: super_class_data_option,
                    };

                    let class = Class {
                        class_data: Rc::new(class_data),
                    };
                    self.current_environment
                        .define(class_name, Some(Types::Class(Rc::new(class))));

                    Ok(())
                }
                _ => unreachable!(),
            },
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
                    closure: self.current_environment.clone(),
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
    Class(Rc<Class>),
    Instance(Instance),
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
            &Types::Boolean(b) => write!(f, "{}", b),
            &Types::Callable(ref c) => write!(f, "{}", c),
            &Types::Class(ref c) => write!(f, "{}", c),
            &Types::Instance(ref instance) => write!(f, "{}", instance),
            &Types::Nil => write!(f, "nil"),
            &Types::Number(n) => write!(f, "{}", n),
            &Types::ReturnString(ref s) => write!(f, "\"{}\"", s.to_string()),
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

impl Function {
    fn bind(&self, instance: Types) -> Function {
        let mut environment = Environment::new_node(&self.closure);
        environment.define(&Token::This, Some(instance));
        Function {
            parameters: self.parameters.clone(),
            body: self.body.clone(),
            closure: environment,
        }
    }
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

#[derive(Debug)]
pub struct Class {
    class_data: Rc<ClassData>,
}

#[derive(Debug)]
struct ClassData {
    name: String,
    methods: HashMap<String, Function>,
    super_class: Option<Rc<ClassData>>,
}

impl ClassData {
    fn find_method(&self, name: &str, instance: &Instance) -> Option<Types> {
        match self.methods.get(name) {
            Some(method) => {
                let new_method = method.bind(Types::Instance(instance.clone()));
                Some(Types::Callable(Rc::new(Box::new(new_method))))
            }
            None => match self.super_class {
                Some(ref super_class) => match super_class.methods.get(name) {
                    Some(method) => {
                        let new_method = method.bind(Types::Instance(instance.clone()));
                        Some(Types::Callable(Rc::new(Box::new(new_method))))
                    }
                    None => None,
                },
                None => None,
            },
        }
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        if let Some(initializer) = self.class_data.methods.get("init") {
            return initializer.arity();
        }
        0
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Types>) -> Result<Types, String> {
        let instance_data = InstanceData {
            fields: HashMap::new(),
        };

        let instance = Types::Instance(Instance {
            class_data: self.class_data.clone(),
            instance_data: Rc::new(RefCell::new(instance_data)),
        });

        if let Some(initializer) = self.class_data.methods.get("init") {
            initializer
                .bind(instance.clone())
                .call(interpreter, arguments)?;
        }

        Ok(instance)
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.class_data.name)
    }
}

#[derive(Debug)]
struct InstanceData {
    fields: HashMap<String, Types>,
}

impl InstanceData {
    fn get(&self, name: &str) -> Option<Types> {
        match self.fields.get(name) {
            Some(return_value) => Some(return_value.clone()),
            None => None,
        }
    }

    fn set(&mut self, name: String, value: Types) {
        self.fields.insert(name, value);
    }
}

#[derive(Debug, Clone)]
pub struct Instance {
    class_data: Rc<ClassData>,
    instance_data: Rc<RefCell<InstanceData>>,
}

impl Instance {
    fn get(&self, name: &str) -> Option<Types> {
        match self.instance_data.borrow().get(name) {
            some @ Some(_) => some,
            None => self.class_data.find_method(name, self),
        }
    }

    fn set(&self, name: String, value: Types) {
        self.instance_data.borrow_mut().set(name, value)
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} instance", self.class_data.name)
    }
}
