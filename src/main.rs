pub mod citation;
pub mod config_file;
pub mod reference;
pub mod pkg_mgr;
pub mod search;

use git2::cert::SshHostKeyType;
use reference::Reference;
use libcanon::search::search;
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
    let mut ref_input = "";
    let mut verbose = false;
    let mut show_nums = false;

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
        } else if arg == "search" {
            match search(&canon_path, &args[i+2]) {
                Err(what) => {println!("{}", what)}
                Ok(results) => {
                    for res in results {
                        println!("{}\t{}", res.reference, res.text.trim());
                    }
                }
            }
            return;
        } else if arg.starts_with('-') {
            for ch in arg[1..].chars() {
                match ch {
                    'v' => {verbose = true;},
                    'n' => {show_nums = true;},
                    x => { println!("Flag not found: {}", x); return; }
                }
            }
        } else {
            ref_input = arg;
        }
    }

    // Parse the reference
    let reference = Reference::from_str(ref_input).unwrap();
    //println!("{:?}", reference);
    let result = cite(&canon_path, &reference);
    match result {
        Ok(citation) => {
            //println!("{:?}", citation);
            if verbose {
                println!("@{}", citation)
            }
            for ch in citation.chapters {
                for v in ch.verses {
                    println!(" {} {}", v.verse, v.content);
                }
            }
        }
        Err(problem) => {
            eprintln!("{}", problem);
        }
    }
}

