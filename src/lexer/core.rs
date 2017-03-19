pub struct Lexer {
    input: Vec<char>,
    curr: usize,
}

impl Iterator for Lexer {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.curr == self.input.len() {
            return None;
        } else {
            self.curr += 1;
            return Some(self.input[self.curr - 1]);
        }
    }
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            curr: 0,
        }
    }
}