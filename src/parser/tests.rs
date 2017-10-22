use ast::*;
use ast_printer::*;
use lexer::*;
use super::*;

#[test]
fn parser_test_1() {
    // Test for the results of parsing the following program:
    // 1 * 2 + -3 >= 4 != true;

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
        Token::Semicolon,
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

    let expected_expr = Expression::Binary(
        Box::new(one_star_two_plus_neg_three_greater_equal_four),
        Token::NotEqual,
        Box::new(true_expr),
    );

    let expected_ast = Statement::Expression(expected_expr);

    let statements = Parser::new(&tokens).parse().unwrap();

    assert_eq!(
        ASTStringVisitor {
            statements: &[expected_ast],
        }.to_string(),
        ASTStringVisitor {
            statements: &statements,
        }.to_string()
    );
}

#[test]
fn parser_test_2() {
    // Test for the results of parsing the following program:
    // 1 * (2 + -3);

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
        Token::Semicolon,
    ];

    let one_expr = Expression::Literal(one_token);
    let two_expr = Expression::Literal(two_token);
    let three_expr = Expression::Literal(three_token);

    let neg_three = Expression::Unary(Token::Minus, Box::new(three_expr));

    let two_plus_neg_three =
        Expression::Binary(Box::new(two_expr), Token::Plus, Box::new(neg_three));

    let two_neg_three_grouping = Expression::Grouping(Box::new(two_plus_neg_three));

    let expected_expr = Expression::Binary(
        Box::new(one_expr),
        Token::Asterisk,
        Box::new(two_neg_three_grouping),
    );

    let expected_ast = Statement::Expression(expected_expr);

    let statements = Parser::new(&tokens).parse().unwrap();

    assert_eq!(
        ASTStringVisitor {
            statements: &[expected_ast],
        }.to_string(),
        ASTStringVisitor {
            statements: &statements,
        }.to_string()
    );
}

#[test]
fn parser_test_3() {
    // Test for the results of parsing the following program:
    // 1 > 2;

    let one_token = Token::Number(1.0);
    let two_token = Token::Number(2.0);

    let tokens = vec![
        one_token.clone(),
        Token::GreaterThan,
        two_token.clone(),
        Token::Semicolon,
    ];

    let one_expr = Expression::Literal(one_token);
    let two_expr = Expression::Literal(two_token);

    let expected_expr =
        Expression::Binary(Box::new(one_expr), Token::GreaterThan, Box::new(two_expr));

    let expected_ast = Statement::Expression(expected_expr);

    let statements = Parser::new(&tokens).parse().unwrap();

    assert_eq!(
        ASTStringVisitor {
            statements: &[expected_ast],
        }.to_string(),
        ASTStringVisitor {
            statements: &statements,
        }.to_string()
    );
}

#[test]
fn parser_print_test() {
    // Test for the results of parsing the following program:
    // print 1;

    let one_token = Token::Number(1.0);

    let tokens = vec![Token::Print, one_token.clone(), Token::Semicolon];

    let one_expr = Expression::Literal(one_token);

    let expected_ast = Statement::Print(one_expr);

    let statements = Parser::new(&tokens).parse().unwrap();

    assert_eq!(
        ASTStringVisitor {
            statements: &[expected_ast],
        }.to_string(),
        ASTStringVisitor {
            statements: &statements,
        }.to_string()
    );
}

#[test]
fn parser_variable_declaration_test_1() {
    // Test for the results of parsing the following program:
    // let a = 1 + 2;

    let a_token = Token::Ident(String::from("a"));
    let one_token = Token::Number(1.0);
    let two_token = Token::Number(2.0);

    let tokens = vec![
        Token::Let,
        a_token.clone(),
        Token::Assign,
        one_token.clone(),
        Token::Plus,
        two_token.clone(),
        Token::Semicolon,
    ];

    let one_expr = Expression::Literal(one_token);
    let two_expr = Expression::Literal(two_token);

    let one_plus_two = Expression::Binary(Box::new(one_expr), Token::Plus, Box::new(two_expr));

    let expected_ast = Statement::VariableDeclaration(a_token, Some(one_plus_two));

    let statements = Parser::new(&tokens).parse().unwrap();

    assert_eq!(
        ASTStringVisitor {
            statements: &[expected_ast],
        }.to_string(),
        ASTStringVisitor {
            statements: &statements,
        }.to_string()
    );
}

#[test]
fn parser_variable_declaration_test_2() {
    // Test for the results of parsing the following program:
    // let a;

    let a_token = Token::Ident("a".to_string());

    let tokens = vec![Token::Let, a_token.clone(), Token::Semicolon];

    let expected_ast = Statement::VariableDeclaration(a_token, None);

    let statements = Parser::new(&tokens).parse().unwrap();

    assert_eq!(
        ASTStringVisitor {
            statements: &[expected_ast],
        }.to_string(),
        ASTStringVisitor {
            statements: &statements,
        }.to_string()
    );
}


#[test]
fn parser_variable_test() {
    // Test for the results of parsing the following program:
    // a;

    let a_token = Token::Ident("a".to_string());

    let tokens = vec![a_token.clone(), Token::Semicolon];

    let a_expr = Expression::Variable(a_token);

    let expected_ast = Statement::Expression(a_expr);

    let statements = Parser::new(&tokens).parse().unwrap();

    assert_eq!(
        ASTStringVisitor {
            statements: &[expected_ast],
        }.to_string(),
        ASTStringVisitor {
            statements: &statements,
        }.to_string()
    );
}

#[test]
fn parser_assignment_test() {
    // Test for the results of parsing the following program:
    // a = 8;

    let a_token = Token::Ident("a".to_string());
    let eight_token = Token::Number(8.0);

    let tokens = vec![
        a_token.clone(),
        Token::Assign,
        eight_token.clone(),
        Token::Semicolon,
    ];

    let eight_expr = Expression::Literal(eight_token);

    let a_assign_eight = Expression::Assignment(a_token, Box::new(eight_expr));

    let expected_ast = Statement::Expression(a_assign_eight);

    let statements = Parser::new(&tokens).parse().unwrap();

    assert_eq!(
        ASTStringVisitor {
            statements: &[expected_ast],
        }.to_string(),
        ASTStringVisitor {
            statements: &statements,
        }.to_string()
    );
}

#[test]
fn block_test() {
    // Test for the results of parsing the following program:
    // {a = 8;}

    let a_token = Token::Ident("a".to_string());
    let eight_token = Token::Number(8.0);

    let tokens = vec![
        Token::LeftBrace,
        a_token.clone(),
        Token::Assign,
        eight_token.clone(),
        Token::Semicolon,
        Token::RightBrace,
    ];

    let eight_expr = Expression::Literal(eight_token);

    let a_assign_eight = Expression::Assignment(a_token, Box::new(eight_expr));

    let a_assign_eight_stmt = Statement::Expression(a_assign_eight);

    let expected_ast = Statement::Block(vec![a_assign_eight_stmt]);

    let statements = Parser::new(&tokens).parse().unwrap();

    assert_eq!(
        ASTStringVisitor {
            statements: &[expected_ast],
        }.to_string(),
        ASTStringVisitor {
            statements: &statements,
        }.to_string()
    );
}
