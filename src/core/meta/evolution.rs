use crate::core::graph::graph::Graph;
use crate::core::graph::node::Node;

pub struct MetaEvolution;

impl MetaEvolution {

    pub fn mutate(graph: &mut Graph) {

        println!("Meta evolution started");

        let next_id: usize = graph.nodes.len() + 1;

        let node = Node {
            id: next_id,
            name: "MetaAdaptiveNode".to_string(),
        };

        graph.nodes.push(node);

        println!("Meta mutation applied");
    }
}