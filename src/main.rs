extern crate catlox;
extern crate liner;

use std::io;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use liner::Context;

use catlox::ast_printer::*;
use catlox::lexer::*;
use catlox::parser::*;
use catlox::interpreter::*;
use catlox::resolver::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_debug = args.contains(&String::from("debug"));
    let files : Vec<&String> = args.iter().filter(|s| s.ends_with(".cbox")).collect();

    if files.len() > 2 {
        unreachable!("cat-lox can only run one file at a time.")
    }

    if files.len() == 1 {
        let mut f = File::open(files[0]).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        let mut interpreter = Interpreter::new(Box::new(|s| println!("{}", s)));
        run(&contents, is_debug, 0, &mut interpreter);
    } else {
        repl(is_debug).unwrap();
    }
}

pub fn repl(is_debug: bool) -> io::Result<()> {
    println!(
        r#"
                   __    _____  _  _
                  (  )  (  _  )( \/ )
  /\-/\            )(__  )(_)(  )  (
 /a a  \          (____)(_____)(_/\_)  _
=\ Y  =/-~~~~~~-,_____________________/ )
  '^--'          ______________________/
    \           /
    ||  |---'\  \
   (_(__|   ((__|

catlox is free software with ABSOLUTELY NO WARRANTY.
"#
    );

    let mut interpreter = Interpreter::new(Box::new(|s| println!("{}", s)));
    let mut con = Context::new();
    let mut parse_seed = 0;

    loop {
        let res = con.read_line("> ", &mut |_| {});

        match res {
            Ok(res) => {
                if let Some(new_parse_seed) = run(&res, is_debug, parse_seed, &mut interpreter) {
                    parse_seed = new_parse_seed;
                }

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

fn run(res: &str, is_debug: bool, parse_seed: usize, interpreter: &mut Interpreter) -> Option<usize> {
    let tokens: Vec<Token> = Lexer::new(res).collect();

    if is_debug {
        println!("Tokens ----");
        for t in &tokens {
            println!("{:?}", t);
        }
    }

    match Parser::new(&tokens, parse_seed).parse() {
        Ok((new_parse_seed, statements)) => {
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
            match resolve(&statements, interpreter) {
                Ok(()) => match interpreter.interpret(&statements) {
                    Ok(_) => (),
                    Err(err) => println!("Interpreter Error: {}", err),
                },
                Err(err) => println!("Resolver Error: {}", err),
            }

            Some(new_parse_seed)
        }
        Err(err) => {
            println!("Parse Error: {}", err);
            None
        }
    }
}
