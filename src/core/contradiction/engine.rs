use crate::core::graph::graph::SemanticGraph;

#[derive(Debug)]
pub struct Contradiction {

    pub node_a: usize,
    pub node_b: usize,
    pub severity: f32,
}

pub struct ContradictionEngine;

impl ContradictionEngine {

    pub fn detect(graph: &SemanticGraph) {

        println!("CONTRADICTION ENGINE");

        for edge in &graph.edges {

            if edge.from == edge.to {

                let contradiction = Contradiction {

                    node_a: edge.from,
                    node_b: edge.to,
                    severity: 1.0,
                };

                println!(
                    "contradiction -> {} <-> {} severity={}",
                    contradiction.node_a,
                    contradiction.node_b,
                    contradiction.severity
                );
            }
        }

        println!("Contradiction scan finished");
    }
}