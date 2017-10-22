use ast::core::*;
use std::io::Write;

pub trait MutVisitor {
    type E;
    type S;

    fn visit_expression(&mut self, e: &Expression) -> Self::E;
    fn visit_statement<W: Write>(&mut self, s: &Statement, w: &mut W) -> Self::S;
}

pub trait Visitor {
    type E;
    type S;

    fn visit_expression(&self, e: &Expression) -> Self::E;
    fn visit_statement(&self, s: &Statement) -> Self::S;
}
