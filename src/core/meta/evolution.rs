use crate::core::graph::graph::SemanticGraph;

pub struct MetaEvolution;

impl MetaEvolution {

    pub fn evolve(graph: &mut SemanticGraph) {

        println!("Meta evolution started");

        let id = graph.add_node("meta");

        if id > 1 {

            graph.add_edge(id - 1, id);
        }

        println!("Meta evolution finished");
    }
}