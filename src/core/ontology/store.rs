use crate::core::ontology::entity::OntologyEntity;

pub struct OntologyStore {
    pub entities: Vec<OntologyEntity>,
}

impl OntologyStore {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn insert(&mut self, entity: OntologyEntity) {
        println!(
            "ontology insert -> {}",
            entity.label
        );

        self.entities.push(entity);
    }

    pub fn print(&self) {
        println!(
            "ontology entities -> {}",
            self.entities.len()
        );

        for entity in &self.entities {
            println!(
                "entity [{}] {} ({})",
                entity.id,
                entity.label,
                entity.category
            );
        }
    }
}