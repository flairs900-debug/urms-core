use crate::core::graph::graph::Graph;

pub struct ReflectionAnalyzer;

impl ReflectionAnalyzer {
    pub fn analyze(
        graph: &Graph,
    ) {
        for node in &graph.nodes {
            println!(
                "reflection => {}",
                node.name
            );
        }
    }
}