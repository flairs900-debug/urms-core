use crate::core::ontology::entity::Entity;

pub struct OntologyStore {
    pub entities: Vec<Entity>,
}

impl OntologyStore {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}