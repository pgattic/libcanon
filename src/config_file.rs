use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
    pub priority: Vec<String>,
}

impl GlobalConfig {
    pub fn from_str(string: String) -> Result<Self> {
        serde_json::from_str(&string)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CanonConfig {
    pub aliases: HashMap<String, Vec<String>>
}

impl CanonConfig {
    /// # Example
    ///
    /// ```
    /// use libcanon::config_file::CanonConfig;
    /// let json_data = r#"
    /// {
    ///   "aliases": {
    ///     "1 Nephi": ["1Ne", "1 Ne", "1Ne.", "1 Ne."],
    ///     "2 Nephi": ["2Ne", "2 Ne", "2Ne.", "2 Ne."],
    ///     "Jacob": ["Jac", "Jac."]
    ///   }
    /// }
    /// "#;
    /// 
    /// // Deserialize JSON into BookAliases struct
    /// let book_aliases: CanonConfig = serde_json::from_str(json_data).expect("Failed to deserialize");
    /// 
    /// // Check if the deserialization worked correctly
    /// assert_eq!(book_aliases.aliases.len(), 3);
    /// assert_eq!(book_aliases.aliases["1 Nephi"], vec!["1Ne", "1 Ne", "1Ne.", "1 Ne."]);
    /// ```
    pub fn from_str(string: String) -> Result<Self> {
        serde_json::from_str(&string)
    }
}


