use ast::*;
use std::fmt;

pub struct ASTStringVisitor<'a> {
    pub expr: &'a Expression,
}

impl<'a> Visitor<String> for ASTStringVisitor<'a> {
    fn visit_expression(&self, e: &Expression) -> String {
        match *e {
            Expression::Literal(ref t) => format!("(Literal {:?})", t),
            Expression::Unary(ref t, ref e) => {
                format!("(Unary {:?} {})", t, self.visit_expression(e))
            }
            Expression::Binary(ref e, ref t, ref e2) => format!(
                "(Binary {:?} {} {})",
                t,
                self.visit_expression(e),
                self.visit_expression(e2)
            ),
            Expression::Grouping(ref e) => format!("(Grouping {})", self.visit_expression(e)),
        }
    }
}

impl<'a> fmt::Display for ASTStringVisitor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.visit_expression(self.expr))
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

        assert_eq!(
            "(Grouping (Binary Plus (Literal Ident(\"1\")) \
             (Unary Minus (Literal Ident(\"2\")))))"
                .to_string(),
            ASTStringVisitor { expr: &expr }.to_string()
        )
    }

    #[test]
    fn to_string_test_2() {
        let one_token = Token::Ident("1".to_string());
        let two_token = Token::Ident("2".to_string());
        let two_expr = Expression::Literal(two_token);

        let mut expr = Expression::Literal(one_token);
        expr = Expression::Binary(Box::new(expr), Token::Plus, Box::new(two_expr));

        assert_eq!(
            "(Binary Plus (Literal Ident(\"1\")) (Literal \
             Ident(\"2\")))"
                .to_string(),
            ASTStringVisitor { expr: &expr }.to_string()
        )
    }
}
