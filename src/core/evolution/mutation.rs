use crate::core::graph::graph::SemanticGraph;

pub struct DynamicMutation;

impl DynamicMutation {

    pub fn mutate(
        graph: &mut SemanticGraph
    ) {

        println!("DYNAMIC MUTATION");

        let id =
            graph.add_node(
                "MutatedNode"
            );

        if id > 1 {

            graph.add_edge(
                id - 1,
                id,
            );
        }

        println!(
            "mutation node -> {}",
            id
        );

        println!("Mutation completed");
    }
}