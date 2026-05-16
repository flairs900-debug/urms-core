use crate::core::graph::graph::SemanticGraph;

pub struct ActivationPropagation;

impl ActivationPropagation {
    pub fn propagate(graph: &SemanticGraph) {
        println!("Activation propagation started");

        for edge in &graph.edges {
            println!("activation -> {} -> {}", edge.from, edge.to);
        }

        println!("Activation propagated");
    }
}