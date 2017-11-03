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
            Expression::Assignment(ref token, ref expr) => {
                format!("(Assignment {:?} {})", token, self.visit_expression(expr))
            }
            Expression::Binary(ref expr1, ref token, ref expr2) => format!(
                "(Binary {:?} {} {})",
                token,
                self.visit_expression(expr1),
                self.visit_expression(expr2)
            ),
            Expression::Call(ref callee, _, ref args) => format!(
                "(Call {} {})",
                self.visit_expression(callee),
                args.iter()
                    .map(|e| self.visit_expression(e))
                    .collect::<String>()
            ),
            Expression::Grouping(ref expr) => format!("(Grouping {})", self.visit_expression(expr)),
            Expression::Literal(ref token) => format!("(Literal {:?})", token),
            Expression::Logical(ref expr1, ref token, ref expr2) => format!(
                "(Logical {:?} {} {})",
                token,
                self.visit_expression(expr1),
                self.visit_expression(expr2)
            ),
            Expression::Unary(ref token, ref expr) => {
                format!("(Unary {:?} {})", token, self.visit_expression(expr))
            }
            Expression::Variable(ref token) => format!("(Variable {:?})", token),
        }
    }

    fn visit_statement(&self, s: &Statement) -> String {
        match *s {
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
            Statement::VariableDeclaration(ref token, ref expr) => format!(
                "(VariableDeclaration Statement {:?} {})",
                token,
                match expr {
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
