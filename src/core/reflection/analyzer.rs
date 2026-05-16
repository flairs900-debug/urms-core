use crate::core::graph::graph::SemanticGraph;

pub struct ReflectionAnalyzer;

impl ReflectionAnalyzer {
    pub fn reflect(graph: &SemanticGraph) {
        println!("Reflection started");

        println!("nodes count -> {}", graph.nodes.len());
        println!("edges count -> {}", graph.edges.len());

        for node in &graph.nodes {
            println!("reflect node -> {}", node.name);
        }

        println!("Reflection finished");
    }
}