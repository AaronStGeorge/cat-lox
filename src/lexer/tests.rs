use super::*;

// TODO: investigate generative testing. This seems like it would be highly
// amenable.
#[test]
fn lexer_test() {
    let input = "

    let five = 5;
    let ten = 10;
    let a_string = \"
    blaaaa
    \";

    let add = fn(x, y) {
        x + y; // Comments ++** 900) () fuck yeah!!
    };

    let result = add(five, ten);
    !-/*5;
    // Comment the fuck out of this bullshit == +++
    5 < 10 > 5;

    if (5 < 10) {
	    return true;
    } else {
	    return false;
    }

    10 == 10;
    10 != 9;

    // Woo comments are the best 8==D
    ";

    let expected = vec![
        Token::Let,
        Token::Ident {
            literal: "five".to_string(),
        },
        Token::Assign,
        Token::Number(5.0),
        Token::Semicolon,
        Token::Let,
        Token::Ident {
            literal: "ten".to_string(),
        },
        Token::Assign,
        Token::Number(10.0),
        Token::Semicolon,
        Token::Let,
        Token::Ident {
            literal: "a_string".to_string(),
        },
        Token::Assign,
        Token::LoxString("\n    blaaaa\n    ".to_string()),
        Token::Semicolon,
        Token::Let,
        Token::Ident {
            literal: "add".to_string(),
        },
        Token::Assign,
        Token::Function,
        Token::LeftParentheses,
        Token::Ident {
            literal: "x".to_string(),
        },
        Token::Comma,
        Token::Ident {
            literal: "y".to_string(),
        },
        Token::RightParentheses,
        Token::LeftBrace,
        Token::Ident {
            literal: "x".to_string(),
        },
        Token::Plus,
        Token::Ident {
            literal: "y".to_string(),
        },
        Token::Semicolon,
        Token::RightBrace,
        Token::Semicolon,
        Token::Let,
        Token::Ident {
            literal: "result".to_string(),
        },
        Token::Assign,
        Token::Ident {
            literal: "add".to_string(),
        },
        Token::LeftParentheses,
        Token::Ident {
            literal: "five".to_string(),
        },
        Token::Comma,
        Token::Ident {
            literal: "ten".to_string(),
        },
        Token::RightParentheses,
        Token::Semicolon,
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Number(5.0),
        Token::Semicolon,
        Token::Number(5.0),
        Token::LessThan,
        Token::Number(10.0),
        Token::GreaterThan,
        Token::Number(5.0),
        Token::Semicolon,
        Token::If,
        Token::LeftParentheses,
        Token::Number(5.0),
        Token::LessThan,
        Token::Number(10.0),
        Token::RightParentheses,
        Token::LeftBrace,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::RightBrace,
        Token::Else,
        Token::LeftBrace,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::RightBrace,
        Token::Number(10.0),
        Token::Equal,
        Token::Number(10.0),
        Token::Semicolon,
        Token::Number(10.0),
        Token::NotEqual,
        Token::Number(9.0),
        Token::Semicolon,
    ];

    let lexer = Lexer::new(input);
    let results: Vec<Token> = lexer.collect();

    for (i, tok) in results.iter().enumerate() {
        assert_eq!(*tok, expected[i]);
    }

    assert_eq!(results.len(), expected.len());
}

#[test]
fn greater_equal_test() {
    let input = "5 >= 10";

    let expected = vec![Token::Number(5.0), Token::GreaterEqual, Token::Number(10.0)];

    let lexer = Lexer::new(input);
    let results: Vec<Token> = lexer.collect();

    for (i, tok) in results.iter().enumerate() {
        assert_eq!(*tok, expected[i]);
    }

    assert_eq!(results.len(), expected.len());
}

#[test]
fn less_equal_test() {
    let input = "5 <= 10";

    let expected = vec![Token::Number(5.0), Token::LessEqual, Token::Number(10.0)];

    let lexer = Lexer::new(input);
    let results: Vec<Token> = lexer.collect();

    for (i, tok) in results.iter().enumerate() {
        assert_eq!(*tok, expected[i]);
    }

    assert_eq!(results.len(), expected.len());
}

#[test]
fn nil_test() {
    let input = "a = nil";

    let expected = vec![
        Token::Ident {
            literal: "a".to_string(),
        },
        Token::Assign,
        Token::Nil,
    ];

    let lexer = Lexer::new(input);
    let results: Vec<Token> = lexer.collect();

    for (i, tok) in results.iter().enumerate() {
        assert_eq!(*tok, expected[i]);
    }

    assert_eq!(results.len(), expected.len());
}

#[test]
fn lox_string_test() {
    let input = "\" bla! \"";

    let expected = vec![Token::LoxString(" bla! ".to_string())];

    let lexer = Lexer::new(input);
    let results: Vec<Token> = lexer.collect();

    for (i, tok) in results.iter().enumerate() {
        assert_eq!(*tok, expected[i]);
    }

    assert_eq!(results.len(), expected.len());
}
