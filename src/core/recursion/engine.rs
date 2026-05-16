use crate::core::graph::graph::SemanticGraph;

pub struct RecursiveEngine;

impl RecursiveEngine {

    pub fn recurse(
        graph: &mut SemanticGraph,
        depth: u32,
    ) {

        println!("RECURSIVE ENGINE");

        for i in 0..depth {

            let node =
                graph.add_node(
                    &format!("RecursiveNode{}", i)
                );

            if node > 1 {

                graph.add_edge(
                    node - 1,
                    node,
                );
            }

            println!(
                "recursive depth -> {}",
                i
            );
        }

        println!("Recursive engine finished");
    }
}