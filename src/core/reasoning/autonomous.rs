use crate::core::graph::graph::SemanticGraph;

pub struct AutonomousReasoner;

impl AutonomousReasoner {

    pub fn think(graph: &mut SemanticGraph) {

        for node in &mut graph.nodes {

            node.weight += 0.05;
        }

        println!("Autonomous reasoning finished");
    }
}