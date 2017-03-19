extern crate poople;

use poople::lexer::*;

fn main() {
    let lexer = Lexer::new("忠犬ハチ公");

    for i in lexer {
        println!("{}", i);
    }
}