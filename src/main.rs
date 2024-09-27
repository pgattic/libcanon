pub mod citation;
pub mod config_file;
pub mod reference;

use citation::*;
use std::env;
use dirs::home_dir;
use std::path::PathBuf;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} [COMMAND/REFERENCE] [OPTS]", args[0]);
        println!("Try \"{} help\" for more information.", args[0]);
        return;
    }

    let canon_path: PathBuf = home_dir().unwrap().join(".canon");

    let result = cite(canon_path, &args[1]);

    match result {
        Ok(citation) => {
            print!("{}", citation.text);
        }
        Err((problem, book)) => {
            eprintln!("{}: {}", problem, book);
        }
    }
}

