use ast::*;
use interpreter::Interpreter;
use std::collections::HashMap;
use lexer::Token;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum FunctionType {
    None,
    Function,
}

pub fn resolve(stmts: &mut [Statement], interpreter: &mut Interpreter) -> Result<(), String> {
    let mut resolver = Resolver {
        interpreter: interpreter,
        scopes: Vec::new(),
        function_type: FunctionType::None,
    };
    resolver.resolve(stmts)?;

    Ok(())
}

struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    function_type: FunctionType,
}

impl<'a> Resolver<'a> {
    fn resolve(&mut self, stmts: &[Statement]) -> Result<(), String> {
        for stmt in stmts {
            self.visit_statement(stmt)?;
        }
        Ok(())
    }

    fn declare(&mut self, name_token: &Token) -> Result<(), String> {
        if self.scopes.is_empty() {
            return Ok(());
        }

        match name_token {
            &Token::Ident(ref name) => {
                let len = self.scopes.len() - 1;
                if self.scopes[len].contains_key(name) {
                    return Err(String::from(
                        "Variable with this name already declared in this scope",
                    ));
                }
                self.scopes[len].insert(name.to_string(), false);
            }
            _ => unreachable!(),
        };
        Ok(())
    }

    fn define(&mut self, name_token: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        match name_token {
            &Token::Ident(ref name) => {
                if self.scopes.is_empty() {
                    return;
                }
                let len = self.scopes.len() - 1;
                self.scopes[len].insert(name.to_string(), true);
            }
            _ => unreachable!(),
        };
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn resolve_local(&mut self, name: &str, expr: &Expression) {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(name) {
                // Scopes does not include the global environment, the resolver
                // will. Add one for this reason.
                self.interpreter.resolve(expr, i + 1);
                return;
            }
        }
    }

    fn resolve_fn(
        &mut self,
        function_stmt: &Statement,
        function_type: FunctionType,
    ) -> Result<(), String> {
        let enclosing_function = self.function_type.clone();
        self.function_type = function_type;
        match function_stmt {
            &Statement::FunctionDeclaration(_, ref parameters, ref body) => {
                self.begin_scope();
                for param in parameters {
                    self.declare(param)?;
                    self.define(param);
                }
                self.resolve(body)?;
                self.end_scope();
            }
            _ => unreachable!(),
        };

        self.function_type = enclosing_function;
        Ok(())
    }
}

impl<'a> MutVisitor for Resolver<'a> {
    type E = Result<(), String>;
    type S = Result<(), String>;

    fn visit_expression(&mut self, e: &Expression) -> Self::E {
        match e {
            &Expression::Assignment {
                ref name, ref expr, ..
            } => {
                self.visit_expression(expr)?;
                let name = match name {
                    &Token::Ident(ref name_s) => name_s,
                    _ => unreachable!(),
                };
                self.resolve_local(name, e);
                Ok(())
            }
            &Expression::Binary {
                ref l_expr,
                ref r_expr,
                ..
            } => {
                self.visit_expression(l_expr)?;
                self.visit_expression(r_expr)?;
                Ok(())
            }
            &Expression::Call {
                ref callee,
                ref arguments,
                ..
            } => {
                self.visit_expression(callee)?;
                for expr in arguments {
                    self.visit_expression(expr)?;
                }
                Ok(())
            }
            &Expression::Get { ref object, .. } => self.visit_expression(object),
            &Expression::Grouping { ref expr, .. } => self.visit_expression(expr),
            &Expression::Literal { .. } => Ok(()),
            &Expression::Logical {
                ref l_expr,
                ref r_expr,
                ..
            } => {
                self.visit_expression(l_expr)?;
                self.visit_expression(r_expr)?;
                Ok(())
            }
            &Expression::Set { .. } => unimplemented!(),
            &Expression::Unary { ref expr, .. } => self.visit_expression(expr),
            &Expression::Variable { ref name, .. } => {
                // We're in the global scope do nothing
                if self.scopes.is_empty() {
                    return Ok(());
                }
                let name = match name {
                    &Token::Ident(ref name_s) => name_s,
                    _ => unreachable!(),
                };
                let len = self.scopes.len() - 1;
                if self.scopes[len].get(name) == Some(&false) {
                    return Err(String::from(
                        "Cannot read local variable in its own initializer.",
                    ));
                }
                self.resolve_local(name, e);
                Ok(())
            }
        }
    }

    fn visit_statement(&mut self, s: &Statement) -> Self::S {
        match s {
            &Statement::Class(ref name, ..) => {
                self.declare(name)?;
                self.define(name);
                Ok(())
            }
            &Statement::Block(ref statements) => {
                self.begin_scope();
                self.resolve(statements)?;
                self.end_scope();
                Ok(())
            }
            &Statement::Expression(ref expr) => {
                self.visit_expression(expr)?;
                Ok(())
            }
            &Statement::FunctionDeclaration(ref name, _, _) => {
                self.declare(name)?;
                self.define(name);
                self.resolve_fn(s, FunctionType::Function)?;
                Ok(())
            }
            &Statement::If(ref condition, ref then, ref else_option) => {
                self.visit_expression(condition)?;
                self.visit_statement(then)?;
                if let &Some(ref stmt) = else_option {
                    self.visit_statement(stmt)?;
                }
                Ok(())
            }
            &Statement::Print(ref expr) => {
                self.visit_expression(expr)?;
                Ok(())
            }
            &Statement::Return(ref expr_option) => {
                if self.function_type == FunctionType::None {
                    return Err(String::from("Cannot return from top level code"));
                }
                if let &Some(ref expr) = expr_option {
                    self.visit_expression(expr)?;
                }
                Ok(())
            }
            &Statement::VariableDeclaration(ref name, ref initializer) => {
                self.declare(name)?;
                match initializer {
                    &Some(ref expr) => self.visit_expression(expr)?,
                    _ => (),
                }
                self.define(name);

                Ok(())
            }
            &Statement::While(ref condition, ref body) => {
                self.visit_expression(condition)?;
                self.visit_statement(body)?;
                Ok(())
            }
        }
    }
}
