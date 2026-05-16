use crate::core::graph::graph::SemanticGraph;

pub struct RewriteEngine;

impl RewriteEngine {

    pub fn rewrite(graph: &mut SemanticGraph) {

        println!("rewrite engine started");

        if let Some(node) = graph.nodes.get_mut(0) {

            node.name = "ReflectiveSemanticEngine".to_string();

            println!(
                "rewrite -> SemanticEngine -> {}",
                node.name
            );
        }

        println!("rewrite engine finished");
    }
}