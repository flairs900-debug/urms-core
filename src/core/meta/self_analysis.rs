use crate::core::graph::graph::SemanticGraph;

pub struct SelfAnalysis;

impl SelfAnalysis {

    pub fn analyze(
        graph: &SemanticGraph
    ) {

        println!("SELF ANALYSIS");

        if graph.nodes.len() < 3 {

            println!(
                "analysis -> weak cognition"
            );

        } else if graph.nodes.len() < 10 {

            println!(
                "analysis -> stable cognition"
            );

        } else {

            println!(
                "analysis -> expanding intelligence"
            );
        }

        println!(
            "node count -> {}",
            graph.nodes.len()
        );

        println!(
            "edge count -> {}",
            graph.edges.len()
        );
    }
}