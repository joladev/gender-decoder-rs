extern crate serde_json;
use std::fs::File;
use std::path::Path;

#[derive(Deserialize)]
pub struct WordLists {
    pub feminine: Vec<String>,
    pub masculine: Vec<String>
}

impl WordLists {
    pub fn new(path: &str) -> WordLists {
        let file = File::open(Path::new(path)).expect(&format!("No path: {}", path));
        serde_json::from_reader(file).expect(&format!("JSON deserialization of word list failed: {}", path))
    }
}
