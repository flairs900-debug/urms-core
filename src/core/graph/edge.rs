use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub relation: String,
}