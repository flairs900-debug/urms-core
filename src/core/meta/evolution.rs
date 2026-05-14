use crate::core::graph::graph::Graph;
use crate::core::graph::node::Node;
use crate::core::graph::edge::Edge;
use crate::core::graph::relation::Relation;

pub struct EvolutionEngine;

impl EvolutionEngine {

    pub fn evolve(graph: &mut Graph) {

        println!("Evolution engine active");

        let mut next_id = graph.nodes.len() as u64 + 1;

        let mut should_expand = false;

        for node in &graph.nodes {

            if node.name == "AdaptiveExecutionEngine" {

                should_expand = true;
            }
        }

        if should_expand {

            println!(
                "AdaptiveExecutionEngine detected"
            );

            graph.add_node(
                Node {

                    id: next_id,

                    name: "CognitiveLayer".to_string(),

                }
            );

            graph.add_edge(
                Edge {

                    from: 3,

                    to: next_id,

                    relation: Relation::EvolvesTo,

                }
            );

            println!(
                "New node created => CognitiveLayer"
            );

            next_id += 1;
        }

        println!("Evolution complete");
    }
}