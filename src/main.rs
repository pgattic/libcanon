pub mod citation;
pub mod config_file;
pub mod reference;
pub mod pkg_mgr;

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
    let mut ref_input = "";

    for (i, arg) in args[1..].iter().enumerate() {
        if arg == "list" {
            let pkgs = pkg_mgr::list(&canon_path).unwrap();
            for pkg in pkgs {
                println!("\x1b[36;1m{}\x1b[0m", pkg);
            }
            return;
        } else if arg == "install" {
            match pkg_mgr::install(&args[i+2], &canon_path) {
                Err(what) => {println!("{}", what)}
                _ => {}
            }
            return;
        } else if arg == "remove" {
            match pkg_mgr::remove(&args[i+2], &canon_path) {
                Err(what) => {println!("{}", what)}
                _ => {}
            }
            return;
        } else {
            ref_input = arg;
        }
    }

    // Parse the reference
    let reference = Reference::from_str(ref_input).unwrap();
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

