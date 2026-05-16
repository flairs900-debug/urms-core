use crate::core::graph::graph::SemanticGraph;

pub struct GraphPruner;

impl GraphPruner {
    pub fn prune(graph: &mut SemanticGraph) {
        println!("GRAPH PRUNER");

        let max_nodes = 32;

        if graph.nodes.len() <= max_nodes {
            println!("pruning skipped");
            return;
        }

        let remove_count = graph.nodes.len() - max_nodes;

        for _ in 0..remove_count {
            if !graph.nodes.is_empty() {
                graph.nodes.remove(0);
            }
        }

        graph.edges.retain(|(a, b)| {
            *a < graph.nodes.len() &&
            *b < graph.nodes.len()
        });

        println!("nodes after prune -> {}", graph.nodes.len());
        println!("edges after prune -> {}", graph.edges.len());
    }
}