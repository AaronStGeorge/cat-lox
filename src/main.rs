extern crate poople;

use poople::lexer::*;

fn main() {
    let lexer = Lexer::new("Aaron");

    for i in lexer {
        println!("{}", i);
    }
}