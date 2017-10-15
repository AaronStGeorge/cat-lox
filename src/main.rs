extern crate poople;

use std::io;

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

    repl::start(io::stdin(), io::stdout()).unwrap();
}
