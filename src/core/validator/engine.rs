use crate::core::graph::graph::SemanticGraph;

pub struct ValidatorEngine;

impl ValidatorEngine {
    pub fn validate(graph: &SemanticGraph) {
        println!("truth valid -> {}", graph.nodes.len() > 0);
    }
}