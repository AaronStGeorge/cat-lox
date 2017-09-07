extern crate poople;

use std::io;

use poople::repl;

fn main() {
    println!(
        "
 _______  _______  _______     _______  _
(  ____ )(  ___  )(  ___  )   (  ____ )( \\
| (    )|| (   ) || (   ) |   | (    )|| (
| (____)|| |   | || |   | |   | (____)|| |
|  _____)| |   | || |   | |   |  _____)| |
| (      | |   | || |   | |   | (      | |
| )      | (___) || (___) | _ | )      | (____/\\
|/       (_______)(_______)(_)|/       (_______/

The programing language that is a total piece of 💩.
Poo.pl is free software with ABSOLUTELY NO WARRANTY.
"
    );

    repl::start(io::stdin(), io::stdout()).unwrap();
}
