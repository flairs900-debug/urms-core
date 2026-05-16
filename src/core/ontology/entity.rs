#[derive(Debug, Clone)]
pub struct OntologyEntity {
    pub id: usize,
    pub label: String,
    pub category: String,
}

impl OntologyEntity {
    pub fn new(
        id: usize,
        label: &str,
        category: &str,
    ) -> Self {
        Self {
            id,
            label: label.to_string(),
            category: category.to_string(),
        }
    }
}