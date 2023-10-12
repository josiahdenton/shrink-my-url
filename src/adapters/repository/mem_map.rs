use std::collections::HashMap;

use anyhow::anyhow;

use crate::domain::minify::SaveMinification;

#[derive(Debug, Clone)]
pub struct MemMap {
    map: HashMap<String, String>,
}

impl MemMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
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
    fn save(&mut self, alias: &str, url: &str) -> anyhow::Result<()> {
        if let Some(_) = self.lookup(alias) {
            return Err(anyhow!("collision"));
        }
        self.add(alias.into(), url.into());
        Ok(())
    }
}
