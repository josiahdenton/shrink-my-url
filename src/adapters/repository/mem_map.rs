use std::collections::HashMap;

use crate::domain::minify::{SaveMinification, SaveResult};


pub struct MemMap {
    map: HashMap<String, String>
}

impl MemMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn lookup(&self, alias: &str) -> Option<&String> {
        self.map.get(alias)
    }

    pub fn add(&mut self, alias: String, url: String) -> Option<String> {
        self.map.insert(alias, url)
    }
}

impl SaveMinification for MemMap {
    fn save(&mut self, alias: String, url: String) -> SaveResult {
        let found = self.add(alias, url);

        match found {
            None => SaveResult::UrlAdded,
            Some(_) => SaveResult::UrlExists
        }
    }
}
