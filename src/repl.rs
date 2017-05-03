use std::io::{self, Write};

use lexer::Lexer;

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
        let lexer = Lexer::new(&buffer);
        for i in lexer {
            println!("{:?}", i);
        }
        stdout.flush()?;
    }
}
