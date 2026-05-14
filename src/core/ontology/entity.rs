#[derive(Clone, Debug)]
pub struct Entity {
    pub name: String,
    pub entity_type: String,
}

impl Entity {
    pub fn new(name: &str, entity_type: &str) -> Self {
        Self {
            name: name.to_string(),
            entity_type: entity_type.to_string(),
        }
    }
}