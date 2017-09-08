use ast::*;
use lexer::*;
use std::fmt;

pub struct ASTStringVisitor<'a> {
    pub expr: &'a Expression,
}

impl<'a> Visitor<String> for ASTStringVisitor<'a> {
    fn visit_expression(&self, e: &Expression) -> String {
        match *e {
            Expression::Literal(ref s) => format!("(Literal {})", s),
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

#[test]
fn to_string_test_1() {
    let one = Box::new(Expression::Literal("1".to_string()));
    let two = Box::new(Expression::Literal("2".to_string()));
    let negative_two = Box::new(Expression::Unary(Box::new(Token::Minus), two));
    let one_plus_negative_two =
        Box::new(Expression::Binary(one, Box::new(Token::Plus), negative_two));
    let ast = Expression::Grouping(one_plus_negative_two);

    assert_eq!(
        "(Grouping (Binary Plus (Literal 1) (Unary Minus (Literal 2))))".to_string(),
        ASTStringVisitor { expr: &ast }.to_string()
    )
}

#[test]
fn to_string_test_2() {
    let mut expr = Box::new(Expression::Literal("1".to_string()));
    let two = Box::new(Expression::Literal("2".to_string()));
    expr = Box::new(Expression::Binary(expr, Box::new(Token::Plus), two));

    assert_eq!(
        "(Binary Plus (Literal 1) (Literal 2))".to_string(),
        ASTStringVisitor { expr: &expr }.to_string()
    )
}
