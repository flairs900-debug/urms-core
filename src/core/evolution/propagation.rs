use crate::core::graph::graph::SemanticGraph;

pub struct ActivationPropagation;

impl ActivationPropagation {
    pub fn propagate(
        graph: &mut SemanticGraph,
    ) {
        println!("activation propagated");

        for node in &mut graph.node_store {
            node.activation += 0.1;
        }
    }
}