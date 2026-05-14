use crate::core::graph::graph::Graph;

pub struct ValidatorEngine;

impl ValidatorEngine {

    pub fn validate(graph: &Graph) {

        println!("Validator active");

        if graph.nodes.is_empty() {

            println!("Validation error: graph has no nodes");

        } else {

            println!(
                "Validation success: {} nodes",
                graph.nodes.len()
            );
        }

        if graph.edges.is_empty() {

            println!("Warning: graph has no edges");

        } else {

            println!(
                "Validation success: {} edges",
                graph.edges.len()
            );
        }
    }
}