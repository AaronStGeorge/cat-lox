use ast::*;
use interpreter::Interpreter;

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
}

impl<'a> MutVisitor for Resolver<'a> {
    type E = ();
    type S = ();

    fn visit_expression(&mut self, e: &Expression) -> Self::E {
        match e {
            &Expression::Assignment(ref token, ref expr) => unimplemented!(),
            &Expression::Binary(ref l_expr, ref token, ref r_expr) => unimplemented!(),
            &Expression::Call(ref callee_expr, ref argument_exprs) => unimplemented!(),
            &Expression::Grouping(ref expr) => unimplemented!(),
            &Expression::Literal(ref token) => unimplemented!(),
            &Expression::Logical(ref l_expr, ref token, ref r_expr) => unimplemented!(),
            &Expression::Unary(ref token, ref expr) => unimplemented!(),
            &Expression::Variable(ref token) => unimplemented!(),
        }
    }

    fn visit_statement(&mut self, s: &Statement) -> Self::S {
        match s {
            &Statement::Block(ref statements) => unimplemented!(),
            &Statement::Expression(ref expr) => unimplemented!(),
            &Statement::FunctionDeclaration(ref name_token, ref parameters, ref body) => {
                unimplemented!()
            }
            &Statement::If(ref conditional_expr, ref then, ref else_option) => unimplemented!(),
            &Statement::Print(ref expr) => unimplemented!(),
            &Statement::Return(ref expr_option) => unimplemented!(),
            &Statement::VariableDeclaration(ref token, ref initialzer) => unimplemented!(),
            &Statement::While(ref expr, ref stmt) => unimplemented!(),
        }
    }
}
