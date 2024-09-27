
use std::ops::RangeInclusive;

#[derive(Clone, Debug, Eq, PartialEq)]
/// Data structure for storing a complete scripture reference.
///
/// Includes the Book name, chapters and verses.
pub struct Reference {
    pub book: String,
    pub indications: Vec<RefIndic>,
}

impl Reference {
    /// Marshals the reference string into the `Reference` data structure.
    ///
    /// # Examples
    /// ```
    /// use libcanon::reference::Reference;
    /// assert_eq!(Reference::from_str("Matthew 7:21").unwrap().book, "Matthew");
    /// assert_eq!(Reference::from_str("Rom8:16-18").unwrap().book, "Rom");
    /// ```
    pub fn from_str(src: &str) -> Option<Self> {
        let src = src.trim();
        let mut book = "";
        let mut indication = "";
        for (i, ch) in src.chars().rev().enumerate() {
            book = &src[..src.len()-i];
            indication = &src[src.len()-i..];
            if !"1234567890 :-,;".contains(ch) {
                break;
            }
        }

        let indications: Vec<RefIndic> = indication
            .split(';')
            .map(&str::trim)
            .map(RefIndic::from_str)
            .into_iter()
            .filter_map(|x| x) // Filter by Some(x)
            .collect();

        Some(Self {
            book: book.to_string(),
            indications
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefIndic {
    pub chapter: String,
    pub verse_ranges: Vec<RefVerse>,
}

impl RefIndic {
    /// Marshals the reference string into the `Reference` data structure.
    ///
    /// # Examples
    ///
    /// ```
    /// use libcanon::reference::RefIndic;
    /// assert_eq!(RefIndic::from_str("1:5-6").unwrap().chapter, "1");
    /// assert_eq!(RefIndic::from_str("5").unwrap().chapter, "5");
    /// ```
    pub fn from_str(src: &str) -> Option<Self> {
        if src.contains(':') {
            let parts: Vec<&str> = src.split(':').collect();
            if parts.len() > 2 {
                return None;
            }
            let verses = parts[1]
                .trim()
                .split(',')
                .filter_map(RefVerse::from_str)
                .collect();
            return Some(Self{chapter: parts[0].trim().to_string(), verse_ranges: verses});
        } else {
            return Some(Self{chapter: src.trim().to_string(), verse_ranges: vec![RefVerse::All] })
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RefVerse {
    All,
    Single(usize),
    Range(RangeInclusive<usize>)
}

impl RefVerse {
    /// Could take "3" or "7-9"
    ///
    /// ```
    /// use libcanon::reference::RefVerse;
    /// assert_eq!(RefVerse::from_str("3"), Some(RefVerse::Single(3)));
    /// assert_eq!(RefVerse::from_str("7-9"), Some(RefVerse::Range(7, 9)));
    /// assert_eq!(RefVerse::from_str("abc"), None);
    /// ```
    pub fn from_str(src: &str) -> Option<Self> {
        if src.contains('-') {
            let parts: Vec<&str> = src.split('-').collect();
            let start = parts.first().unwrap().trim().parse::<usize>().ok()?;
            let end = parts.last().unwrap().trim().parse::<usize>().ok()?;
            return Some(Self::Range(start..=end));
        } else {
            src.parse::<usize>().ok().map(Self::Single)
        }
    }
}
