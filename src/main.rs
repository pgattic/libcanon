pub mod citation;
pub mod config_file;
pub mod reference;
pub mod pkg_mgr;
pub mod search;

use reference::Reference;
use libcanon::search::search;
use citation::*;
use std::env;
use dirs::home_dir;
use std::path::PathBuf;

fn display_help(program: &str) {
    println!("Usage: {} [COMMAND/REFERENCE] [OPTS]", program);
    println!();
    println!("Canon Book Referencer");
    println!();
    println!("Commands:");
    println!("  help                  Display this page");
    println!("  install [Repo URL]    Install canon package from repository");
    println!("  list                  List installed Canon packages");
    println!("  remove [Package]      Remove package by shortname");
    println!();
    println!("If anything besides these commands is given, the input is assumed to be a book");
    println!("reference, followed or preceded by any combination of these reference options:");
    println!();
    println!("Reference Options");
    println!("  -n, --numbered        Print verse/paragraph numbers before each line.");
    println!("  -p, --paragraph       Print the lines of text with an extra space in between");
    println!("                        them, as paragraphs.");
    println!("  -v, --verbose         Print extra information, like where the book was found.");
    println!("                        Useful for supplementary tools like canonmk. It is not");
    println!("                        recommended to use -n or -p with -v.");
    println!();
    println!("Canon made by Preston Corless (pgattic), free under the MIT License.");
    println!("More information can be found at https://github.com/pgattic/libcanon");

}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} [COMMAND/REFERENCE] [OPTS]", args[0]);
        println!("Try \"{} help\" for more information.", args[0]);
        return;
    }

    let canon_path: PathBuf = home_dir().unwrap().join(".canon");
    let mut ref_input = "";
    let mut show_nums = false;
    let mut paragraphs = false;
    let mut verbose = false;

    for (i, arg) in args[1..].iter().enumerate() {
        if arg == "list" {
            let pkgs = pkg_mgr::list(&canon_path.join("texts")).unwrap();
            for pkg in pkgs {
                println!("\x1b[36;1m{}\x1b[0m", pkg);
            }
            return;
        } else if arg == "install" {
            match pkg_mgr::install(&args[i+2], &canon_path.join("texts")) {
                Err(what) => {println!("{}", what)}
                _ => {}
            }
            return;
        } else if arg == "remove" {
            match pkg_mgr::remove(&args[i+2], &canon_path.join("texts")) {
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
        } else if arg == "help" {
            display_help(&args[0]);
            return;
        } else if arg.starts_with('-') {
            match &arg[..] {
                "--numbered" => {show_nums = true;},
                "--paragraph" => {paragraphs = true;},
                "--verbose" => {verbose = true;},
                not_found if not_found.starts_with("--") => { println!("Flag not recognized: {}", not_found); return; }
                _ => {
                    for ch in arg[1..].chars() {
                        match ch {
                            'n' => {show_nums = true;},
                            'p' => {paragraphs = true;},
                            'v' => {verbose = true;},
                            x => { println!("Flag not recognized: {}", x); return; },
                        }
                    }
                }
            }
        } else {
            ref_input = &arg;
        }
    }


    // Parse the reference
    let reference = Reference::from_str(ref_input).unwrap();
    let result = cite(&canon_path.join("texts"), &reference);
    match result {
        Ok(citation) => {
            if verbose {
                println!("@{}", citation.book_path.to_str().unwrap());
            }
            for ch in citation.chapters {
                if verbose {
                    println!("@@{}", ch.path.to_str().unwrap());
                }
                for v in ch.verses {
                    if verbose {
                        println!("@@@{} {}", v.verse, v.content);
                    } else {
                        if show_nums {
                            println!(" {} {}", v.verse, v.content);
                        } else {
                            println!("{}", v.content);
                        }
                        if paragraphs {
                            println!();
                        }
                    }
                }
            }
        }
        Err(problem) => {
            eprintln!("{}", problem);
        }
    }
}

