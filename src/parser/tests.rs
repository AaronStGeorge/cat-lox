use ast::*;
use ast_printer::*;
use lexer::*;
use super::*;

#[test]
fn parser_test_1() {
    // Test for the results of parsing the following program:
    // 1 * 2 + -3 >= 4 != true

    let one_token = Token::Number(1.0);
    let two_token = Token::Number(2.0);
    let three_token = Token::Number(3.0);
    let four_token = Token::Number(4.0);

    let tokens = vec![
        one_token.clone(),
        Token::Asterisk,
        two_token.clone(),
        Token::Plus,
        Token::Minus,
        three_token.clone(),
        Token::GreaterEqual,
        four_token.clone(),
        Token::NotEqual,
        Token::True,
    ];

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

    let expected_ast = Expression::Binary(
        Box::new(one_star_two_plus_neg_three_greater_equal_four),
        Token::NotEqual,
        Box::new(true_expr),
    );

    let ast = Parser::new(&tokens).parse().unwrap();

    assert_eq!(
        ASTStringVisitor {
            expr: &expected_ast,
        }.to_string(),
        ASTStringVisitor { expr: &ast }.to_string()
    );
}

#[test]
fn parser_test_2() {
    // Test for the results of parsing the following program:
    // 1 * (2 + -3)

    let one_token = Token::Number(1.0);
    let two_token = Token::Number(2.0);
    let three_token = Token::Number(2.0);

    let tokens = vec![
        one_token.clone(),
        Token::Asterisk,
        Token::LeftParentheses,
        two_token.clone(),
        Token::Plus,
        Token::Minus,
        three_token.clone(),
        Token::RightParentheses,
    ];

    let one_expr = Expression::Literal(one_token);
    let two_expr = Expression::Literal(two_token);
    let three_expr = Expression::Literal(three_token);

    let neg_three = Expression::Unary(Token::Minus, Box::new(three_expr));

    let two_plus_neg_three =
        Expression::Binary(Box::new(two_expr), Token::Plus, Box::new(neg_three));

    let two_neg_three_grouping = Expression::Grouping(Box::new(two_plus_neg_three));

    let expected_ast = Expression::Binary(
        Box::new(one_expr),
        Token::Asterisk,
        Box::new(two_neg_three_grouping),
    );

    let ast = Parser::new(&tokens).parse().unwrap();

    assert_eq!(
        ASTStringVisitor {
            expr: &expected_ast,
        }.to_string(),
        ASTStringVisitor { expr: &ast }.to_string()
    );
}

#[test]
fn parser_test_3() {
    // Test for the results of parsing the following program:
    // 1 > 2

    let one_token = Token::Number(1.0);
    let two_token = Token::Number(2.0);

    let tokens = vec![one_token.clone(), Token::GreaterThan, two_token.clone()];

    let one_expr = Expression::Literal(one_token);
    let two_expr = Expression::Literal(two_token);

    let expected_ast =
        Expression::Binary(Box::new(one_expr), Token::GreaterThan, Box::new(two_expr));

    let ast = Parser::new(&tokens).parse().unwrap();

    assert_eq!(
        ASTStringVisitor {
            expr: &expected_ast,
        }.to_string(),
        ASTStringVisitor { expr: &ast }.to_string()
    );
}
