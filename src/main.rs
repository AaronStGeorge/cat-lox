extern crate poople;

use poople::lexer::Lexer;

fn main() {
    let lexer = Lexer::new("{ }();+,=let 90; fn() 忠犬ハチ公");

    for i in lexer {
        println!("{:?}", i);
    }
}