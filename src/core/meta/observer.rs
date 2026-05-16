use crate::core::graph::graph::SemanticGraph;

pub struct MetaObserver;

impl MetaObserver {

    pub fn inspect(
        graph: &SemanticGraph
    ) {

        println!("META OBSERVER");

        println!(
            "nodes -> {}",
            graph.nodes.len()
        );

        println!(
            "edges -> {}",
            graph.edges.len()
        );

        let density =
            if graph.nodes.len() > 0 {

                graph.edges.len() as f32
                    / graph.nodes.len() as f32

            } else {

                0.0
            };

        println!(
            "graph density -> {}",
            density
        );

        if density > 1.5 {

            println!(
                "meta warning -> unstable growth"
            );
        }

        println!("Meta observer finished");
    }
}