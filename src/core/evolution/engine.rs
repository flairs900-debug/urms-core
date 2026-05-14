use crate::core::graph::graph::Graph;
use crate::core::graph::node::Node;
use crate::core::graph::edge::Edge;

pub struct EvolutionEngine;

impl EvolutionEngine {

    pub fn evolve(graph: &mut Graph) {

        println!("Evolution engine active");

        let mut next_id: usize = graph.nodes.len() + 1;

        let adaptive_node = Node {
            id: next_id,
            name: "AdaptiveLayer".to_string(),
        };

        graph.nodes.push(adaptive_node);

        graph.edges.push(
            Edge {
                from: 1,
                to: next_id as usize,
                relation: "EvolvesTo".to_string(),
            
			});

        println!("Adaptive layer created");

        next_id += 1;

        let cognitive_node = Node {
            id: next_id,
            name: "CognitiveMutation".to_string(),
        };

        graph.nodes.push(cognitive_node);

        graph.edges.push(
            Edge {
                from: 1,
                to: next_id as usize,
                relation: "EvolvesTo".to_string(),
            
			});

        println!("Cognitive mutation created");

        println!("Evolution complete");
    }
}