use ast::core::*;

pub trait Visitor<T> {
    fn visit_expression(&self, e: &Expression) -> T;
}
