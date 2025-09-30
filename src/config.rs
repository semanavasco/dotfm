use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub name: String,
    pub author: String,
    pub files: HashMap<String, PathBuf>,
}
