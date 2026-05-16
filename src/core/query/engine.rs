use crate::core::graph::graph::SemanticGraph;

pub struct QueryEngine;

impl QueryEngine {

    pub fn query(graph: &SemanticGraph) {

        println!("Query engine started");

        for edge in &graph.edges {

            println!(
                "relation -> {} -> {}",
                edge.from,
                edge.to
            );
        }

        println!("Query engine finished");
    }
}