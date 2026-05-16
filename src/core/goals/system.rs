use crate::core::graph::graph::{Edge, Node, SemanticGraph};

pub struct GoalSystem;

impl GoalSystem {
    pub fn initialize(graph: &mut SemanticGraph) {
        graph.nodes.push(Node {
            id: 100,
            name: "Goal".to_string(),
            activation: 1.0,
            weight: 1.0,
        });

        let a = 1;
        let b = 100;

        graph.edges.push(Edge {
            from: a,
            to: b,
        });

        println!("Goal system initialized");
    }
}