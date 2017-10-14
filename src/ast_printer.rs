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
            Expression::Binary(ref expr, ref t, ref e2) => format!(
                "(Binary {:?} {} {})",
                t,
                self.visit_expression(expr),
                self.visit_expression(e2)
            ),
            Expression::Grouping(ref expr) => format!("(Grouping {})", self.visit_expression(expr)),
            Expression::Literal(ref t) => format!("(Literal {:?})", t),
            Expression::Unary(ref t, ref expr) => {
                format!("(Unary {:?} {})", t, self.visit_expression(expr))
            }
            Expression::Variable(ref name) => format!("(Variable {:?})", name),
        }
    }

    fn visit_statement(&self, s: &Statement) -> String {
        match *s {
            Statement::Expression(ref expr) => {
                format!("(Statement Expression {})", self.visit_expression(expr))
            }
            Statement::Print(ref expr) => {
                format!("(Statement Print {})", self.visit_expression(expr))
            }
            Statement::VariableDeclaration(ref name, ref expr) => format!(
                "(Statement VariableDeclaration {} {})",
                name,
                match expr {
                    &Some(ref expr) => self.visit_expression(expr),
                    &None => "nil".to_string(),
                }
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
            "(Statement Expression (Grouping (Binary Plus (Literal Ident(\"1\")) \
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
            "(Statement Expression (Binary Plus (Literal Ident(\"1\")) (Literal \
             Ident(\"2\"))))"
                .to_string(),
            ASTStringVisitor {
                statements: &[stmt],
            }.to_string()
        )
    }
}
