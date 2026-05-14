use crate::core::ontology::store::OntologyStore;

pub struct ReasonerEngine;

impl ReasonerEngine {
    pub fn infer(ontology: &OntologyStore) {
        println!("Reasoning started");

        for entity in &ontology.entities {
            match entity.kind.as_str() {
                "concept" => {
                    println!("Concept detected: {}", entity.name);
                }

                "memory" => {
                    println!("Memory node: {}", entity.name);
                }

                _ => {
                    println!("Unknown entity: {}", entity.name);
                }
            }
        }
    }
}