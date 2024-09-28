
use std::fs;
use std::path::PathBuf;
use crate::reference::*;
use crate::config_file::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Citation {
    //pub reference: Reference,
    pub chapters: Vec<Chapter>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Chapter {
    pub path: PathBuf,
    pub name: String,
    pub verses: Vec<Verse>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Verse {
    //pub reference: Reference,
    //pub path: PathBuf,
    pub verse: usize,
    pub content: String,
}

/// This function searches the user's canon directory for the reference given.
pub fn cite(path: &PathBuf, reference: &Reference) -> Result<Citation, String> {

    let book_path = find_book(path, &reference.book).unwrap();

    let mut res_chs: Vec<Chapter> = vec![];

    for ref_ind in &reference.indications {
        let ch_path = book_path.join(&ref_ind.chapter);
        res_chs.push(Chapter::from_ch_reference(ch_path, &ref_ind.verse_ranges));
    }

    Ok(
        Citation { chapters: res_chs}
    )
}

pub fn find_book(path: &PathBuf, reference: &str) -> Result<PathBuf, String> {

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
                .into_iter()
                .map(|s| s.to_lowercase())
                .collect();

            if lowers.contains(&reference.to_lowercase()) { // Found the book!
                return Ok(text_path.join(book));
            }
        }
    }
    Err("Book not found".to_string())
}

impl Chapter {
    pub fn from_ch_reference(ch_path: PathBuf, references: &Vec<RefVerse>) -> Self {
        let mut result = Self {
            name: "".to_string(),
            path: ch_path.clone(),
            verses: vec![],
        };

        let ch_file = fs::read_to_string(&ch_path).unwrap();
        let split: &Vec<&str> = &ch_file.split('\n').collect();

        for r in references {
            match r {
                RefVerse::All => {
                    for (i, verse) in split.into_iter().enumerate() {
                        result.verses.push(
                            Verse {
                                verse: i + 1,
                                content: verse.to_string()
                            }
                        );
                    }
                }
                RefVerse::Single(verse_num) => {
                    result.verses.push(
                        Verse {
                            verse: *verse_num,
                            content: split[verse_num-1].to_string()
                        }
                    );
                }
                RefVerse::Range(v_range) => {
                    for i in v_range.clone().into_iter() {
                        result.verses.push(
                            Verse {
                                verse: i,
                                content: split[i-1].to_string()
                            }
                        );
                    }
                }
            }
        }
        result
    }
}


