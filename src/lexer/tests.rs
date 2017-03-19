use super::*;

#[test]

fn it_totally_works() {
    let input = "=+(){}";
    let expected: Vec<(&str, &str)> =
        vec![(ASSIGN, "="), (LPAREN, "("), (RPAREN, ")"), (LBRACE, "{"), (RBRACE, "}")];
    let lexer = Lexer::new(input);
}