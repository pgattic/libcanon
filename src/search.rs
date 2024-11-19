//use std::fmt;
use grep_regex::RegexMatcher;
use grep_searcher::{sinks::UTF8, Searcher};
use ignore::WalkBuilder;
use std::path::Path;
use crate::reference::{RefIndic, Reference, RefVerse};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchResult {
    pub text: String,
    pub reference: Reference,
}

pub fn search(path: &Path, pattern: &str) -> Result<Vec<SearchResult>, &'static str> {
    // Create a regex matcher for the pattern
    let mut items = vec![];

    let matcher = match RegexMatcher::new_line_matcher(pattern) {
        Err(_) => {return Err("Failed to parse search term(s)")},
        Ok(mat) => mat
    };

    // Initialize the searcher
    let mut searcher = Searcher::new();

    // Walk through the directory, respecting .gitignore and other ignore files
    let walker = WalkBuilder::new(path)
        .standard_filters(true) // respects ignore files, skips hidden files
        .build();

    for result in walker {
        let entry = match result {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Error reading entry: {}", err);
                continue;
            }
        };

        // Skip directories; we only want files
        if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
            continue;
        }

        // Perform the search on each file path
        let _ = searcher.search_path(
            &matcher,
            entry.path(),
            UTF8(|line_number, line| {
                //items.push(format!("Match found in {}:{}: {}", entry.path().display(), line_number, line));
                let (book, chapter) = get_last_two_dirs(entry.path()).unwrap();
                items.push(SearchResult{
                    text: line.to_string(),
                    reference: Reference {
                        book,
                        indications: vec![
                            RefIndic {
                                chapter,
                                verse_ranges: vec![
                                    RefVerse::Single(line_number as usize)
                                ]
                            }
                        ]
                    }
                });
                Ok(true)
            }),
        );
    }

    Ok(items)
}

fn get_last_two_dirs(path: &Path) -> Option<(String, String)> {
    let components: Vec<_> = path.components().filter_map(|c| {
        match c {
            std::path::Component::Normal(name) => name.to_str(),
            _ => None,
        }
    }).collect();

    if components.len() >= 2 {
        Some((
            components[components.len() - 2].to_string(),
            components[components.len() - 1].to_string(),
        ))
    } else {
        None
    }
}

