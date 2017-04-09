extern crate poople;

use poople::lexer::Lexer;

fn main() {
    let input = "{ }();+,=let 90; fn() 忠犬ハチ公";
    let lexer = Lexer::new(input);

    for i in lexer {
        println!("{:?}", i);
    }
}