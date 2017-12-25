use ast::*;
use interpreter::Interpreter;
use std::collections::HashMap;
use lexer::Token;

pub fn resolve(stmts: &[Statement], interpreter: &mut Interpreter) -> Result<(), String> {
    let mut resolver = Resolver {
        interpreter: interpreter,
        scopes: Vec::new(),
    };
    resolver.resolve(stmts)?;

    Ok(())
}


struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl<'a> Resolver<'a> {
    fn resolve(&mut self, stmts: &[Statement]) -> Result<(), String> {
        for stmt in stmts {
            self.visit_statement(stmt)?;
        }
        Ok(())
    }

    fn declare(&mut self, name_token: &Token) {
        match name_token {
            &Token::Ident(ref name) => {
                if self.scopes.is_empty() {
                    return;
                }
                let len = self.scopes.len() - 1;
                self.scopes[len].insert(name.to_string(), false);
            }
            _ => unreachable!(),
        };
    }

    fn define(&mut self, name_token: &Token) {
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
            if self.scopes[i].contains_key(name) {}
        }
    }

    fn resolve_fn(&mut self, function_stmt: &Statement) -> Result<(), String> {
        match function_stmt {
            &Statement::FunctionDeclaration(_, ref parameters, ref body) => {
                self.begin_scope();
                for param in parameters {
                    self.declare(param);
                    self.define(param);
                }
                self.resolve(body)?;
                self.end_scope();
            }
            _ => unreachable!(),
        };

        Ok(())
    }
}

impl<'a> MutVisitor for Resolver<'a> {
    type E = Result<(), String>;
    type S = Result<(), String>;

    fn visit_expression(&mut self, e: &Expression) -> Self::E {
        match e {
            &Expression::Assignment(ref token, ref expr) => {
                self.visit_expression(expr)?;
                let name = match token {
                    &Token::Ident(ref name_s) => name_s,
                    _ => unreachable!(),
                };
                self.resolve_local(name, expr);
                Ok(())
            }
            &Expression::Binary(ref l_expr, _, ref r_expr) => {
                self.visit_expression(l_expr)?;
                self.visit_expression(r_expr)?;
                Ok(())
            }
            &Expression::Call(ref callee_expr, ref argument_exprs) => {
                self.visit_expression(callee_expr)?;
                for expr in argument_exprs {
                    self.visit_expression(expr)?;
                }
                Ok(())
            }
            &Expression::Grouping(ref inner) => self.visit_expression(inner),
            &Expression::Literal(_) => Ok(()),
            &Expression::Logical(ref l_expr, _, ref r_expr) => {
                self.visit_expression(l_expr)?;
                self.visit_expression(r_expr)?;
                Ok(())
            }
            &Expression::Unary(_, ref expr) => self.visit_expression(expr),
            &Expression::Variable(ref name_token) => {
                let name = match name_token {
                    &Token::Ident(ref name_s) => name_s,
                    _ => unreachable!(),
                };
                let len = self.scopes.len() - 1;
                if self.scopes.is_empty() || self.scopes[len].get(name) != Some(&true) {
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
                self.declare(name);
                self.define(name);
                self.resolve_fn(s)?;
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
                if let &Some(ref expr) = expr_option {
                    self.visit_expression(expr)?;
                }
                Ok(())
            }
            &Statement::VariableDeclaration(ref name, ref initializer) => {
                self.declare(name);
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
