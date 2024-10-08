pub mod citation;
pub mod config_file;
pub mod reference;

use reference::Reference;
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
    //let canon_path: PathBuf = PathBuf::from_str("/home/pgattic/.canon").unwrap();

    // Parse the reference
    let reference = Reference::from_str(&args[1]).unwrap();
    println!("{:?}", reference);
    let result = cite(&canon_path, &reference);
    match result {
        Ok(citation) => {
            println!("{:?}", citation);
        }
        Err(problem) => {
            eprintln!("{}", problem);
        }
    }
}

