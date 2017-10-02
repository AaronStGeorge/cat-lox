use ast::core::*;

pub trait Visitor {
    type E;
    type S;

    fn visit_expression(&self, e: &Expression) -> Self::E;
    fn visit_statement(&self, s: &Statement) -> Self::S;
}
