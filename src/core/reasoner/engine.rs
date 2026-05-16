use crate::core::ontology::entity::OntologyEntity;

pub struct ReasonerEngine;

impl ReasonerEngine {

    pub fn analyze(entity: &OntologyEntity) {

        match entity.category.as_str() {

            "concept" => {
                println!("Concept detected: {}", entity.label);
            }

            "memory" => {
                println!("Memory node: {}", entity.label);
            }

            _ => {
                println!("Unknown entity: {}", entity.label);
            }
        }
    }
}