use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionRecord {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvolutionHistory {
    pub records: Vec<EvolutionRecord>,
}

impl EvolutionHistory {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    pub fn push(&mut self, from: String, to: String) {
        self.records.push(EvolutionRecord {
            from,
            to,
        });
    }
}