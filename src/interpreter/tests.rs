use ast::*;
use lexer::*;
use super::*;

#[test]
fn interpreter_test_1() {
    // Test for the results of interpreting the following program:
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

    let interpreter_result = Interpreter::new(&ast).interpret();
    assert_eq!(interpreter_result.is_ok(), true);
    assert_eq!(interpreter_result.unwrap(), "true");
}

#[test]
fn interpreter_test_2() {
    // Test for the results of interpreting the following program:
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

    let interpreter_result = Interpreter::new(&ast).interpret();
    assert_eq!(interpreter_result.is_ok(), true);
    assert_eq!(interpreter_result.unwrap(), "-1");
}

#[test]
fn interpreter_test_3() {
    // Test for the results of interpreting the following program:
    // 1 > 2

    let one_token = Token::Number(1.0);
    let two_token = Token::Number(2.0);

    let one_expr = Expression::Literal(one_token);
    let two_expr = Expression::Literal(two_token);

    let ast = Expression::Binary(Box::new(one_expr), Token::GreaterThan, Box::new(two_expr));

    let interpreter_result = Interpreter::new(&ast).interpret();
    assert_eq!(interpreter_result.is_ok(), true);
    assert_eq!(interpreter_result.unwrap(), "false");
}

#[test]
fn interpreter_test_4() {
    // Test for the results of interpreting the following program:
    // !nil == true

    let true_expr = Expression::Literal(Token::True);
    let nil_expr = Expression::Literal(Token::Nil);

    let bang_nil = Expression::Unary(Token::Bang, Box::new(nil_expr));

    let ast = Expression::Binary(Box::new(bang_nil), Token::Equal, Box::new(true_expr));

    let interpreter_result = Interpreter::new(&ast).interpret();
    assert_eq!(interpreter_result.is_ok(), true);
    assert_eq!(interpreter_result.unwrap(), "true");
}

#[test]
fn interpreter_test_5() {
    // Test for the results of interpreting the following program:
    // !2 == false

    let two_token = Token::Number(2.0);

    let two_expr = Expression::Literal(two_token);
    let false_expr = Expression::Literal(Token::False);

    let bang_two = Expression::Unary(Token::Bang, Box::new(two_expr));

    let ast = Expression::Binary(Box::new(bang_two), Token::Equal, Box::new(false_expr));

    let interpreter_result = Interpreter::new(&ast).interpret();
    assert_eq!(interpreter_result.is_ok(), true);
    assert_eq!(interpreter_result.unwrap(), "true");
}
