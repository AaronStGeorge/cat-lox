use super::*;

#[test]
fn control_characters() {
    let input = "=+(){}";
    let expected = vec![Token::Assign,
                        Token::Plus,
                        Token::LeftParentheses,
                        Token::RightParentheses,
                        Token::LeftBrace,
                        Token::RightBrace];

    let lexer = Lexer::new(input);
    let results: Vec<Token> = lexer.collect();
    assert_eq!(results.len(), expected.len());

    for (i, tok) in results.iter().enumerate() {
        assert_eq!(*tok, expected[i]);
    }
}

#[test]
fn ints_fns_lets_idents() {
    let input = "

    let five = 5;
    let ten = 10;

    let add = fn(x, y) {
        x + y;
    };

    let result = add(five, ten);";

    let expected = vec![Token::Let,
                        Token::Ident { literal: "five".to_string() },
                        Token::Assign,
                        Token::Int { literal: "5".to_string() },
                        Token::Semicolon,
                        Token::Let,
                        Token::Ident { literal: "ten".to_string() },
                        Token::Assign,
                        Token::Int { literal: "10".to_string() },
                        Token::Semicolon,
                        Token::Let,
                        Token::Ident { literal: "add".to_string() },
                        Token::Assign,
                        Token::Function,
                        Token::LeftParentheses,
                        Token::Ident { literal: "x".to_string() },
                        Token::Comma,
                        Token::Ident { literal: "y".to_string() },
                        Token::RightParentheses,
                        Token::LeftBrace,
                        Token::Ident { literal: "x".to_string() },
                        Token::Plus,
                        Token::Ident { literal: "y".to_string() },
                        Token::Semicolon,
                        Token::RightBrace,
                        Token::Semicolon,
                        Token::Let,
                        Token::Ident { literal: "result".to_string() },
                        Token::Assign,
                        Token::Ident { literal: "add".to_string() },
                        Token::LeftParentheses,
                        Token::Ident { literal: "five".to_string() },
                        Token::Comma,
                        Token::Ident { literal: "ten".to_string() },
                        Token::RightParentheses,
                        Token::Semicolon];

    let lexer = Lexer::new(input);
    let results: Vec<Token> = lexer.collect();
    assert_eq!(results.len(), expected.len());

    for (i, tok) in results.iter().enumerate() {
        assert_eq!(*tok, expected[i]);
    }
}