use crate::core::graph::graph::SemanticGraph;

pub struct EvolutionEngine;

impl EvolutionEngine {

    pub fn evolve(graph: &mut SemanticGraph) {

        let id = graph.add_node("evolved");

        if id > 1 {

            graph.add_edge(id - 1, id);
        }

        println!("Evolution applied");
    }
}