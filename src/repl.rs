extern crate liner;

use std::io;

use self::liner::Context;

use ast_printer::*;
use lexer::*;
use parser::*;
use interpreter::*;

pub fn start(mut stdout: io::Stdout, is_debug: bool) -> io::Result<()> {
    let mut interpreter = Interpreter::new();


    let mut con = Context::new();

    loop {
        let res = con.read_line("> ", &mut |_| {});

        match res {
            Ok(res) => {

                let tokens: Vec<Token> = Lexer::new(&res).collect();

                if is_debug {
                    println!("Tokens ----");
                    for t in &tokens {
                        println!("{:?}", t);
                    }
                }

                let statements = Parser::new(&tokens).parse().unwrap();

                if is_debug {
                    println!("AST ----");
                    println!(
                        "{}",
                        ASTStringVisitor {
                            statements: &statements,
                        }
                    );
                    println!("Output ----");
                }

                interpreter.interpret(&statements, &mut stdout);


                con.history.push(res.into())?;
            }
            Err(e) => {
                match e.kind() {
                    // ctrl-d or ctrl-c pressed
                    io::ErrorKind::UnexpectedEof | io::ErrorKind::Interrupted => {
                        println!("exiting...");
                        break;
                    }
                    _ => {
                        panic!("error: {:?}", e)
                    }
                }
            }
        }
    }

    Ok(())
}
