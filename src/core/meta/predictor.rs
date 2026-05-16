use crate::core::graph::graph::SemanticGraph;

pub struct FuturePredictor;

impl FuturePredictor {

    pub fn predict(
        graph: &SemanticGraph
    ) {

        println!("FUTURE PREDICTOR");

        let future_nodes =
            graph.nodes.len() + 5;

        let future_edges =
            graph.edges.len() + 8;

        println!(
            "predicted nodes -> {}",
            future_nodes
        );

        println!(
            "predicted edges -> {}",
            future_edges
        );

        println!(
            "system trend -> expansion"
        );
    }
}