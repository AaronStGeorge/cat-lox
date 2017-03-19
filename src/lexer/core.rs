use std::str::Chars;

pub struct Lexer<'a> {
    input: Chars<'a>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.input.next() {
            Some(curr) => return Some(curr.to_string()),
            None => return None,
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer { input: input.chars() }
    }
}