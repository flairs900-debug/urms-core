use crate::core::ontology::store::OntologyStore;

pub struct ReasonerEngine;

impl ReasonerEngine {
    pub fn run(ontology: &OntologyStore) {
        for entity in &ontology.entities {
            match entity.entity_type.as_str() {
                "Concept" => {
                    println!("Concept: {}", entity.name);
                }

                "Rule" => {
                    println!("Rule: {}", entity.name);
                }

                "Memory" => {
                    println!("Memory: {}", entity.name);
                }

                _ => {
                    println!("Unknown: {}", entity.name);
                }
            }
        }
    }
}