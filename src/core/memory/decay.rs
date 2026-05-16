use crate::core::graph::graph::SemanticGraph;

pub struct MemoryDecay;

impl MemoryDecay {
    pub fn apply(graph: &mut SemanticGraph) {
        println!("Memory decay applied");

        for node in &graph.nodes {
            println!("memory node -> {}", node.name);
        }
    }
}