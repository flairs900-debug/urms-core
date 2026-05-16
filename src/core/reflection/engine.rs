use crate::core::graph::graph::SemanticGraph;

pub struct ReflectionEngine;

impl ReflectionEngine {

    pub fn reflect(graph: &SemanticGraph) {

        println!("Reflection started");

        println!("nodes count -> {}", graph.nodes.len());

        println!("edges count -> {}", graph.edges.len());

        let avg_weight =
            graph.nodes
                .iter()
                .map(|n| n.weight)
                .sum::<f32>()
                / graph.nodes.len() as f32;

        println!("average weight -> {}", avg_weight);

        println!("Reflection finished");
    }
}