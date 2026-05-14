use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OntologyEntity {
    pub name: String,
    pub kind: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OntologyStore {
    pub entities: Vec<OntologyEntity>,
}

impl OntologyStore {

    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, name: &str, kind: &str) {
        self.entities.push(
            OntologyEntity {
                name: name.to_string(),
                kind: kind.to_string(),
            }
        );
    }

    pub fn save(&self, path: &str) {

        let json =
            serde_json::to_string_pretty(self)
                .expect("Failed serialize ontology");

        std::fs::write(path, json)
            .expect("Failed save ontology");
    }
}