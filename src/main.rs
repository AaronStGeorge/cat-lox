extern crate catbox;
extern crate liner;

use std::io;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use liner::Context;

use catbox::ast_printer::*;
use catbox::lexer::*;
use catbox::parser::*;
use catbox::interpreter::*;
use catbox::resolver::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_debug = args.len() >= 2 && args[1] == String::from("debug");

    if let Some(filename) = match is_debug {
        true => if args.len() == 3 && args[2].ends_with(".cbox") {
            Some(args[2].clone())
        } else {
            None
        },
        false => if args.len() == 2 && args[1].ends_with(".cbox") {
            Some(args[1].clone())
        } else {
            None
        },
    } {
        let mut f = File::open(filename).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        let mut interpreter = Interpreter::new();
        run(&contents, is_debug, &mut interpreter);
    } else {
        repl(is_debug).unwrap();
    }
}

pub fn repl(is_debug: bool) -> io::Result<()> {
    println!(
        r#"
  ___    __   ____  ____  _____  _  _
 / __)  /__\ (_  _)(  _ \(  _  )( \/ )
( (__  /(__)\  )(   ) _ < )(_)(  )  (
 \___)(__)(__)(__) (____/(_____)(_/\_)

                             ,
      ,-.       _,---._ __  / \
     /  )    .-'       `./ /   \
    (  (   ,'            `/    /|
     \  `-"             \'\   / |
      `.              ,  \ \ /  |
       /`.          ,'-`----Y   |
      (            ;        |   '
      |  ,-.    ,-'         |  /
      |  | (   |            | /
      )  |  \  `.___________|/
      `--'   `--'

catbox is free software with ABSOLUTELY NO WARRANTY.
"#
    );

    let mut interpreter = Interpreter::new();
    let mut con = Context::new();

    loop {
        let res = con.read_line("> ", &mut |_| {});

        match res {
            Ok(res) => {
                run(&res, is_debug, &mut interpreter);

                con.history.push(res.into())?;
            }
            Err(e) => {
                match e.kind() {
                    // ctrl-d or ctrl-c pressed
                    io::ErrorKind::UnexpectedEof | io::ErrorKind::Interrupted => {
                        println!("exiting...");
                        break;
                    }
                    _ => panic!("error: {:?}", e),
                }
            }
        }
    }

    Ok(())
}


fn run(res: &str, is_debug: bool, interpreter: &mut Interpreter) {
    let tokens: Vec<Token> = Lexer::new(res).collect();

    if is_debug {
        println!("Tokens ----");
        for t in &tokens {
            println!("{:?}", t);
        }
    }

    match Parser::new(&tokens).parse() {
        Ok(statements) => {
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

            // Resolve variable bindings
            resolve(&statements, interpreter);

            interpreter.interpret(&statements);
        }
        Err(err) => println!("Parse Error: {}", err),
    }
}
