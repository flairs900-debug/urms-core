use crate::core::graph::graph::SemanticGraph;

pub struct GraphState;

impl GraphState {
    pub fn print(graph: &SemanticGraph) {
        println!("GRAPH STATE");

        println!("nodes -> {}", graph.nodes.len());
        println!("edges -> {}", graph.edges.len());

        for node in &graph.nodes {
            println!(
                "NODE [{}] {} activation={} weight={}",
                node.id,
                node.name,
                node.activation,
                node.weight
            );
        }

        for edge in &graph.edges {
            println!("EDGE {} -> {}", edge.from, edge.to);
        }
    }
}