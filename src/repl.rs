use std::io::{self, Write};

use ast_printer::*;
use lexer::*;
use parser::*;
use interpreter::*;

static PROMPT: &'static str = ">> ";

pub fn start(stdin: io::Stdin, mut stdout: io::Stdout) -> io::Result<()> {
    loop {
        // Write prompt
        stdout.write(PROMPT.as_bytes())?;
        stdout.flush()?;

        // Read input
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;

        // Write the results of lexing
        let tokens: Vec<Token> = Lexer::new(&buffer).collect();

        println!("Tokens ----");
        for t in &tokens {
            println!("{:?}", t);
        }

        println!("AST ----");
        let statements = Parser::new(&tokens).parse().unwrap();
        println!(
            "{}",
            ASTStringVisitor {
                statements: &statements,
            }
        );

        println!("Output ----");
        match Interpreter::new(&statements).interpret() {
            Ok(_) => (),
            Err(err) => println!("Run Time Error: {}", err),
        }

        stdout.flush()?;
    }
}
