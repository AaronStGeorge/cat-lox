extern crate poople;

use std::io;
use std::env;

use poople::repl;

fn main() {
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

    let args: Vec<String> = env::args().collect();
    let is_debug = args.len() >= 2 && args[1] == String::from("debug");

    repl::start(io::stdin(), io::stdout(), is_debug).unwrap();
}
