use ast::*;
use std::fmt;

pub struct ASTStringVisitor<'a> {
    pub statements: &'a [Statement],
}

impl<'a> Visitor for ASTStringVisitor<'a> {
    type E = String;
    type S = String;

    fn visit_expression(&self, expr: &Expression) -> String {
        match *expr {
            Expression::Assignment {
                ref name, ref expr, ..
            } => format!("(Assignment {:?} {})", name, self.visit_expression(expr)),
            Expression::Binary {
                ref l_expr,
                ref operator,
                ref r_expr,
                ..
            } => format!(
                "(Binary {:?} {} {})",
                operator,
                self.visit_expression(l_expr),
                self.visit_expression(r_expr)
            ),
            Expression::Call {
                ref callee,
                ref arguments,
                ..
            } => format!(
                "(Call {} {})",
                self.visit_expression(callee),
                arguments
                    .iter()
                    .map(|e| self.visit_expression(e))
                    .collect::<String>()
            ),
            Expression::Get {
                ref name,
                ref object,
                ..
            } => format!("(Get {:?} {})", name, self.visit_expression(object)),
            Expression::Grouping { ref expr, .. } => {
                format!("(Grouping {})", self.visit_expression(expr))
            }
            Expression::Literal { ref token, .. } => format!("(Literal {:?})", token),
            Expression::Logical {
                ref l_expr,
                ref operator,
                ref r_expr,
                ..
            } => format!(
                "(Logical {:?} {} {})",
                operator,
                self.visit_expression(l_expr),
                self.visit_expression(r_expr)
            ),
            Expression::Set {
                ref name,
                ref object,
                ref value,
                ..
            } => format!(
                "(Set \n\t name: {:?} \n\t object: {} \n\t value {})",
                name,
                self.visit_expression(object),
                self.visit_expression(value)
            ),
            Expression::Unary {
                ref operator,
                ref expr,
                ..
            } => format!("(Unary {:?} {})", operator, self.visit_expression(expr)),
            Expression::Variable { ref name, .. } => format!("(Variable {:?})", name),
        }
    }

    fn visit_statement(&self, s: &Statement) -> String {
        match *s {
            Statement::Class(ref name, ref methods) => format!(
                "(ClassDeclaration Statement \n\tname: {:?} \n\tmethods: [{}])",
                name,
                methods.iter()
                    .map(|s| self.visit_statement(s))
                    .collect::<Vec<_>>()
                    .join(", "),
            ),

            Statement::Block(ref statements) => format!(
                "(Block Statement {})",
                statements
                    .iter()
                    .map(|s| self.visit_statement(s))
                    .collect::<String>()
            ),
            Statement::Expression(ref expr) => {
                format!("(Expression Statement {})", self.visit_expression(expr))
            }
            Statement::FunctionDeclaration(ref name, ref parameters, ref body) => format!(
                "(FunctionDeclaration Statement \n\tname: {:?} \n\tparameters: [{}] \n\tbody: {} \n)",
                name,
                parameters
                    .iter()
                    .map(|t| format!("{:?}", t))
                    .collect::<Vec<_>>()
                    .join(", "),
                body.iter()
                    .map(|s| self.visit_statement(s))
                    .collect::<String>()
            ),
            Statement::If(ref conditional, ref then_stmt, ref else_stmt) => format!(
                "(If Statement {} {} {})",
                self.visit_expression(conditional),
                self.visit_statement(then_stmt),
                match else_stmt {
                    &Some(ref inner_else) => self.visit_statement(inner_else),
                    &None => String::from(""),
                }
            ),
            Statement::Print(ref expr) => {
                format!("(Print Statement {})", self.visit_expression(expr))
            }
            Statement::Return(ref expr_option) => format!("(Return Statement {})",
                match expr_option {
                    &Some(ref expr) => self.visit_expression(expr),
                    &None => "nil".to_string(),
                }
            ),
            Statement::VariableDeclaration(ref token, ref expr_option) => format!(
                "(VariableDeclaration Statement {:?} {})",
                token,
                match expr_option {
                    &Some(ref expr) => self.visit_expression(expr),
                    &None => "nil".to_string(),
                }
            ),
            Statement::While(ref expr, ref stmt) => format!(
                "(While Statement {} {})",
                self.visit_expression(expr),
                self.visit_statement(stmt)
            ),
        }
    }
}

impl<'a> fmt::Display for ASTStringVisitor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for s in self.statements {
            write!(f, "{}", self.visit_statement(s))?;
        }
        Ok(())
    }
}
