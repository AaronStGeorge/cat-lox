use std::io::Cursor;

use ast::*;
use lexer::*;
use super::*;

#[test]
fn interpreter_test_1() {
    // Test for the results of interpreting the following expression:
    // 1 * 2 + -3 >= 4 != true

    let one_token = Token::Number(1.0);
    let two_token = Token::Number(2.0);
    let three_token = Token::Number(2.0);
    let four_token = Token::Number(2.0);

    let one_expr = Expression::Literal(one_token);
    let two_expr = Expression::Literal(two_token);
    let three_expr = Expression::Literal(three_token);
    let four_expr = Expression::Literal(four_token);
    let true_expr = Expression::Literal(Token::True);

    let neg_three = Expression::Unary(Token::Minus, Box::new(three_expr));

    let one_star_two = Expression::Binary(Box::new(one_expr), Token::Asterisk, Box::new(two_expr));

    let one_star_two_plus_neg_three =
        Expression::Binary(Box::new(one_star_two), Token::Plus, Box::new(neg_three));

    let one_star_two_plus_neg_three_greater_equal_four = Expression::Binary(
        Box::new(one_star_two_plus_neg_three),
        Token::GreaterEqual,
        Box::new(four_expr),
    );

    let ast = Expression::Binary(
        Box::new(one_star_two_plus_neg_three_greater_equal_four),
        Token::NotEqual,
        Box::new(true_expr),
    );

    let interpreter_result = Interpreter::new().evaluate(&ast);
    assert_eq!(interpreter_result.is_ok(), true);
    assert_eq!(format!("{}", interpreter_result.unwrap()), "true");
}

#[test]
fn interpreter_test_2() {
    // Test for the results of interpreting the following expression:
    // 1 * (2 + -3)

    let one_token = Token::Number(1.0);
    let two_token = Token::Number(2.0);
    let three_token = Token::Number(3.0);

    let one_expr = Expression::Literal(one_token);
    let two_expr = Expression::Literal(two_token);
    let three_expr = Expression::Literal(three_token);

    let neg_three = Expression::Unary(Token::Minus, Box::new(three_expr));

    let two_plus_neg_three =
        Expression::Binary(Box::new(two_expr), Token::Plus, Box::new(neg_three));

    let two_neg_three_grouping = Expression::Grouping(Box::new(two_plus_neg_three));

    let ast = Expression::Binary(
        Box::new(one_expr),
        Token::Asterisk,
        Box::new(two_neg_three_grouping),
    );

    let interpreter_result = Interpreter::new().evaluate(&ast);
    assert_eq!(interpreter_result.is_ok(), true);
    assert_eq!(format!("{}", interpreter_result.unwrap()), "-1");
}

#[test]
fn interpreter_test_3() {
    // Test for the results of interpreting the following expression:
    // 1 > 2

    let one_token = Token::Number(1.0);
    let two_token = Token::Number(2.0);

    let one_expr = Expression::Literal(one_token);
    let two_expr = Expression::Literal(two_token);

    let ast = Expression::Binary(Box::new(one_expr), Token::GreaterThan, Box::new(two_expr));

    let interpreter_result = Interpreter::new().evaluate(&ast);
    assert_eq!(interpreter_result.is_ok(), true);
    assert_eq!(format!("{}", interpreter_result.unwrap()), "false");
}

#[test]
fn interpreter_test_4() {
    // Test for the results of interpreting the following expression:
    // !nil == true

    let true_expr = Expression::Literal(Token::True);
    let nil_expr = Expression::Literal(Token::Nil);

    let bang_nil = Expression::Unary(Token::Bang, Box::new(nil_expr));

    let ast = Expression::Binary(Box::new(bang_nil), Token::Equal, Box::new(true_expr));

    let interpreter_result = Interpreter::new().evaluate(&ast);
    assert_eq!(interpreter_result.is_ok(), true);
    assert_eq!(format!("{}", interpreter_result.unwrap()), "true");
}

#[test]
fn interpreter_test_5() {
    // Test for the results of interpreting the following expression:
    // !2 == false

    let two_token = Token::Number(2.0);

    let two_expr = Expression::Literal(two_token);
    let false_expr = Expression::Literal(Token::False);

    let bang_two = Expression::Unary(Token::Bang, Box::new(two_expr));

    let ast = Expression::Binary(Box::new(bang_two), Token::Equal, Box::new(false_expr));

    let interpreter_result = Interpreter::new().evaluate(&ast);
    assert_eq!(interpreter_result.is_ok(), true);
    assert_eq!(format!("{}", interpreter_result.unwrap()), "true");
}

#[test]
fn variable_declaration_test_1() {
    // Test for the results of interpreting the following statement:
    // let a = 2;
    // Then evaluating the following expression:
    // a

    let mut interpreter = Interpreter::new();

    let a_token = Token::Ident(String::from("a"));
    let two_token = Token::Number(2.0);
    let two_expr = Expression::Literal(two_token);

    let statement_ast = Statement::VariableDeclaration(a_token.clone(), Some(two_expr));

    interpreter.interpret(&[statement_ast], &mut Cursor::new(vec![]));

    let expression_ast = Expression::Variable(a_token);

    let interpreter_result = interpreter.evaluate(&expression_ast);
    assert_eq!(interpreter_result.is_ok(), true);
    assert_eq!(format!("{}", interpreter_result.unwrap()), "2");
}

#[test]
fn variable_assignment_test_1() {
    // Test for the results of interpreting the following statement:
    // let a = 2;
    // a = 3;
    // Then evaluating the following expression:
    // a

    let mut interpreter = Interpreter::new();

    let a_token = Token::Ident(String::from("a"));
    let two_token = Token::Number(2.0);
    let three_token = Token::Number(3.0);

    let two_expr = Expression::Literal(two_token);
    let three_expr = Expression::Literal(three_token);

    let assignment_expr = Expression::Assignment(a_token.clone(), Box::new(three_expr));

    let variable_declaration_stmt = Statement::VariableDeclaration(a_token.clone(), Some(two_expr));
    let assignment_stmt = Statement::Expression(assignment_expr);

    interpreter.interpret(
        &[variable_declaration_stmt, assignment_stmt],
        &mut Cursor::new(vec![]),
    );

    let expression_ast = Expression::Variable(a_token);

    let interpreter_result = interpreter.evaluate(&expression_ast);
    assert_eq!(interpreter_result.is_ok(), true);
    assert_eq!(format!("{}", interpreter_result.unwrap()), "3");
}

#[test]
fn scope_test_1() {
    // Test for the results of interpreting the following program:
    // var a = "global a";
    // var b = "global b";
    // var c = "global c";
    // {
    //   var a = "outer a";
    //   var b = "outer b";
    //   {
    //     var a = "inner a";
    //     print a;
    //     print b;
    //     print c;
    //   }
    //   print a;
    //   print b;
    //   print c;
    // }
    // print a;
    // print b;
    // print c;

    let a_token = Token::Ident(String::from("a"));
    let b_token = Token::Ident(String::from("b"));
    let c_token = Token::Ident(String::from("c"));

    let a_var_expr = Expression::Variable(a_token.clone());
    let b_var_expr = Expression::Variable(b_token.clone());
    let c_var_expr = Expression::Variable(c_token.clone());


    let global_a_token = Token::LoxString(String::from("global a"));
    let global_b_token = Token::LoxString(String::from("global b"));
    let global_c_token = Token::LoxString(String::from("global c"));

    let global_a_expr = Expression::Literal(global_a_token);
    let global_b_expr = Expression::Literal(global_b_token);
    let global_c_expr = Expression::Literal(global_c_token);

    let global_a_declaration = Statement::VariableDeclaration(a_token.clone(), Some(global_a_expr));
    let global_b_declaration = Statement::VariableDeclaration(b_token.clone(), Some(global_b_expr));
    let global_c_declaration = Statement::VariableDeclaration(c_token.clone(), Some(global_c_expr));


    let outer_a_token = Token::LoxString(String::from("outer a"));
    let outer_b_token = Token::LoxString(String::from("outer b"));

    let outer_a_expr = Expression::Literal(outer_a_token);
    let outer_b_expr = Expression::Literal(outer_b_token);

    let outer_a_declaration = Statement::VariableDeclaration(a_token.clone(), Some(outer_a_expr));
    let outer_b_declaration = Statement::VariableDeclaration(b_token.clone(), Some(outer_b_expr));


    let inner_a_token = Token::LoxString(String::from("inner a"));
    let inner_a_expr = Expression::Literal(inner_a_token);
    let inner_a_declaration = Statement::VariableDeclaration(a_token.clone(), Some(inner_a_expr));


    let print_a = Statement::Print(a_var_expr);
    let print_b = Statement::Print(b_var_expr);
    let print_c = Statement::Print(c_var_expr);

    let inner_block = Statement::Block(vec![
        inner_a_declaration,
        print_a.clone(),
        print_b.clone(),
        print_c.clone(),
    ]);

    let outer_block = Statement::Block(vec![
        outer_a_declaration,
        outer_b_declaration,
        inner_block,
        print_a.clone(),
        print_b.clone(),
        print_c.clone(),
    ]);

    let statements = vec![
        global_a_declaration,
        global_b_declaration,
        global_c_declaration,
        outer_block,
        print_a,
        print_b,
        print_c,
    ];

    let mut interpreter = Interpreter::new();
    let mut buff = Cursor::new(vec![]);

    interpreter.interpret(&statements, &mut buff);

    let output = String::from_utf8(buff.into_inner()).unwrap();

    let expected_output = r#""inner a"
"outer b"
"global c"
"outer a"
"outer b"
"global c"
"global a"
"global b"
"global c"
"#;

    assert_eq!(output, expected_output);
}

#[test]
fn if_else_test_1() {
    // Test for the results of interpreting the following program:
    // if (true) print true; else print false;

    let conditional = Expression::Literal(Token::True);
    let then_stmt = Statement::Print(Expression::Literal(Token::True));
    let else_stmt = Statement::Print(Expression::Literal(Token::False));

    let stmt = Statement::If(conditional, Box::new(then_stmt), Some(Box::new(else_stmt)));

    let mut interpreter = Interpreter::new();
    let mut buff = Cursor::new(vec![]);

    interpreter.interpret(&[stmt], &mut buff);

    let output = String::from_utf8(buff.into_inner()).unwrap();

    let expected_output = "true\n";

    assert_eq!(output, expected_output);
}

#[test]
fn if_else_test_2() {
    // Test for the results of interpreting the following program:
    // if (true) print true; else print false;

    let conditional = Expression::Literal(Token::False);
    let then_stmt = Statement::Print(Expression::Literal(Token::True));
    let else_stmt = Statement::Print(Expression::Literal(Token::False));

    let stmt = Statement::If(conditional, Box::new(then_stmt), Some(Box::new(else_stmt)));

    let mut interpreter = Interpreter::new();
    let mut buff = Cursor::new(vec![]);

    interpreter.interpret(&[stmt], &mut buff);

    let output = String::from_utf8(buff.into_inner()).unwrap();

    let expected_output = "false\n";

    assert_eq!(output, expected_output);
}

#[test]
fn logical_test_1() {
    // Test for the results of interpreting the following expression:
    // true and false

    let true_expr = Expression::Literal(Token::True);
    let false_expr = Expression::Literal(Token::False);

    let ast = Expression::Logical(Box::new(true_expr), Token::LogicAnd, Box::new(false_expr));

    let interpreter_result = Interpreter::new().evaluate(&ast);
    assert_eq!(interpreter_result.is_ok(), true);
    assert_eq!(format!("{}", interpreter_result.unwrap()), "false");
}

#[test]
fn logical_test_2() {
    // Test for the results of interpreting the following expression:
    // true or false

    let true_expr = Expression::Literal(Token::True);
    let false_expr = Expression::Literal(Token::False);

    let ast = Expression::Logical(Box::new(true_expr), Token::LogicOr, Box::new(false_expr));

    let interpreter_result = Interpreter::new().evaluate(&ast);
    assert_eq!(interpreter_result.is_ok(), true);
    assert_eq!(format!("{}", interpreter_result.unwrap()), "true");
}

#[test]
fn while_test_1() {
    // Test for the results of interpreting the following expression:
    // let a = 9;
    // while(a > 1) a = a - 1;
    // Then evaluating the following expression:
    // a

    // interpret let a = 9;

    let mut interpreter = Interpreter::new();

    let a_token = Token::Ident(String::from("a"));
    let a_expr = Expression::Variable(a_token.clone());

    let nine_expr = Expression::Literal(Token::Number(9.0));

    let variable_declaration_stmt =
        Statement::VariableDeclaration(a_token.clone(), Some(nine_expr));

    interpreter.interpret(&[variable_declaration_stmt], &mut Cursor::new(vec![]));

    // interpret while(a > 1) a = a - 1;

    let one_expr = Expression::Literal(Token::Number(1.0));

    let a_greater_than_one = Expression::Binary(
        Box::new(a_expr.clone()),
        Token::GreaterThan,
        Box::new(one_expr.clone()),
    );

    let a_minus_one = Expression::Binary(
        Box::new(a_expr.clone()),
        Token::Minus,
        Box::new(one_expr.clone()),
    );

    let assignment_expr = Expression::Assignment(a_token.clone(), Box::new(a_minus_one));
    let assignment_stmt = Statement::Expression(assignment_expr);

    let while_stmt = Statement::While(a_greater_than_one, Box::new(assignment_stmt));

    interpreter.interpret(&[while_stmt], &mut Cursor::new(vec![]));

    // evaluate a

    let expression_ast = Expression::Variable(a_token);

    let interpreter_result = interpreter.evaluate(&expression_ast);

    // check the results

    assert_eq!(interpreter_result.is_ok(), true);
    assert_eq!(format!("{}", interpreter_result.unwrap()), "1");
}
