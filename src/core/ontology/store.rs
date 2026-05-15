use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OntologyStore {
    pub entities: HashMap<String, String>,
}

impl OntologyStore {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, kind: String) {
        self.entities.insert(name, kind);
    }

    pub fn get_kind(&self, name: &str) -> Option<&String> {
        self.entities.get(name)
    }
}