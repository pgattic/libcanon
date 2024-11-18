use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct GlobalConfig {
    pub priority: Vec<String>,
}

impl GlobalConfig {
    /// Loads canon's global configuration file for package management
    pub fn load(path: &PathBuf) -> Result<Self, &'static str> {

        // Directory where Canon stores its texts and global config
        let texts_path = path.join("texts");

        // Read and marshal Canon's global config file
        return match fs::read_to_string(texts_path.join("config.json")) {
            Ok(data) => match serde_json::from_str(&data) {
                Ok(config) => Ok(config),
                Err(_) => {
                    return Err("Malformed Canon config");
                }
            }
            Err(_) => { // Let's just make a new config then yeah
                return Ok(Self{priority: vec![]});
            }
        }
    }

    pub fn store(&self, path: &PathBuf) -> Result<(), &'static str> {
        // Directory where Canon stores its texts and global config
        let texts_path = path.join("texts");

        let file = match serde_json::to_string(self) {
            Ok(data) => data,
            Err(_) => {
                return Err("Failed to create Global Config file");
            }
        };
        match fs::write(texts_path.join("config.json"), file) {
            Ok(()) => {},
            Err(_) => {
                return Err("Failed to write Global Config File");
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageConfig {
    pub aliases: HashMap<String, Vec<String>>
}

impl PackageConfig {
    /// # Example
    ///
    /// ```
    /// use libcanon::config_file::PackageConfig;
    /// let json_data = r#"
    /// {
    ///   "aliases": {
    ///     "1 Nephi": ["1Ne", "1 Ne", "1Ne.", "1 Ne."],
    ///     "2 Nephi": ["2Ne", "2 Ne", "2Ne.", "2 Ne."],
    ///     "Jacob": ["Jac", "Jac."]
    ///   }
    /// }"#;
    /// 
    /// // Deserialize JSON into BookAliases struct
    /// let book_aliases: PackageConfig = serde_json::from_str(json_data).expect("Failed to deserialize");
    /// 
    /// // Check if the deserialization worked correctly
    /// assert_eq!(book_aliases.aliases.len(), 3);
    /// assert_eq!(book_aliases.aliases["1 Nephi"], vec!["1Ne", "1 Ne", "1Ne.", "1 Ne."]);
    /// ```
    pub fn from_str(string: String) -> serde_json::Result<Self> {
        serde_json::from_str(&string)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct PackageManagerConfig {
    pub priority: Vec<String>,
}

impl PackageManagerConfig {

    pub fn from_str(string: String) -> serde_json::Result<Self> {
        serde_json::from_str(&string)
    }

    pub fn to_str(value: Self) -> serde_json::Result<String> {
        serde_json::to_string(&value)
    }
}

