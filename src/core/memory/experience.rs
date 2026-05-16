use crate::core::graph::graph::SemanticGraph;

pub struct ExperienceMemory;

impl ExperienceMemory {
    pub fn store(graph: &SemanticGraph) {
        println!("EXPERIENCE MEMORY");

        for node in &graph.nodes {
            println!(
                "memory node -> {} weight={}",
                node.name,
                node.weight
            );
        }
    }
}