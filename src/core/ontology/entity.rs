use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OntologyEntity {
    pub name: String,
    pub kind: String,
}