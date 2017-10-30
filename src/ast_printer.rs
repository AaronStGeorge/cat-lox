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

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::*;

    #[test]
    fn to_string_test_1() {
        let one_token = Token::Ident("1".to_string());
        let two_token = Token::Ident("2".to_string());

        let one_expr = Expression::Literal(one_token);
        let two_expr = Expression::Literal(two_token);
        let negative_two_expr = Expression::Unary(Token::Minus, Box::new(two_expr));
        let one_plus_negative_two_expr =
            Expression::Binary(Box::new(one_expr), Token::Plus, Box::new(negative_two_expr));
        let expr = Expression::Grouping(Box::new(one_plus_negative_two_expr));

        let stmt = Statement::Expression(expr);

        assert_eq!(
            "(Expression Statement (Grouping (Binary Plus (Literal Ident(\"1\")) \
             (Unary Minus (Literal Ident(\"2\"))))))"
                .to_string(),
            ASTStringVisitor {
                statements: &[stmt],
            }.to_string()
        )
    }

    #[test]
    fn to_string_test_2() {
        let one_token = Token::Ident("1".to_string());
        let two_token = Token::Ident("2".to_string());
        let two_expr = Expression::Literal(two_token);

        let mut expr = Expression::Literal(one_token);
        expr = Expression::Binary(Box::new(expr), Token::Plus, Box::new(two_expr));

        let stmt = Statement::Expression(expr);

        assert_eq!(
            "(Expression Statement (Binary Plus (Literal Ident(\"1\")) (Literal \
             Ident(\"2\"))))"
                .to_string(),
            ASTStringVisitor {
                statements: &[stmt],
            }.to_string()
        )
    }
}
