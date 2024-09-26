
use std::fs;
use std::path::PathBuf;
use crate::reference::*;
use crate::config_file::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Citation {
    pub reference: Reference,
    pub book: String, // The looked-up book name; distinct from the reference's book attribute
    pub text: String, // This should definitely be expanded upon in the future.
}

/// Mama mia this function sure is a piece-a spaghetti!!
/// This function searches the user's canon directory for the reference given.
pub fn cite(path: PathBuf, reference: &str) -> Result<Citation, (&str, String)> {
    // Parse the reference
    let reference = Reference::from_str(reference).unwrap();

    // Directory where Canon stores its texts and global config
    let texts_path = path.join("texts");
    
    // Read and marshal Canon's global config file
    let config_contents = fs::read_to_string(texts_path.join("config.json")).unwrap();
    let config = GlobalConfig::from_str(config_contents).unwrap();

    // Search the installed canons in order of their priority
    for dirname in config.priority {
        let text_path = texts_path.join(dirname);

        // Read and marshal the canon's config (stores book aliases)
        let text_config_contents = fs::read_to_string(text_path.join("config.json"))
            .unwrap();
        let canon_config = CanonConfig::from_str(text_config_contents).unwrap();

        // See if any of the aliases match the input reference
        for (book, aliases) in canon_config.aliases {
            // Case-insensitive search
            let lowers: Vec<String> = aliases
                .clone()
                .into_iter()
                .map(|s| s.to_lowercase())
                .collect();

            if lowers.contains(&reference.book.to_lowercase()) { // Found the book!
                let mut res_text = "".to_string();
                for chapter in &reference.indications {
                    let ch_file = fs::read_to_string(text_path.join(&book).join(&chapter.chapter)).unwrap();
                    match &chapter.verses {
                        None => { // `None` really means "all" (if no verse is specified, get entire chapter)
                            res_text.push_str(&ch_file);
                        }
                        Some(v_refs) => {
                            for r in v_refs {
                                match r {
                                    RefVerse::Single(verse_num) => {
                                        res_text.push_str(&ch_file.split('\n').collect::<Vec<_>>()[verse_num-1])
                                    }
                                    RefVerse::Range(start, end, ) => {
                                        for verse_num in *start..*end {
                                            res_text.push_str(&ch_file.split('\n').collect::<Vec<_>>()[verse_num-1])
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                return Ok(Citation {
                    reference,
                    book: book.to_string(),
                    text: res_text,
                });
            }
        }
    }
    Err(("Book not found", reference.book.clone()))

}


