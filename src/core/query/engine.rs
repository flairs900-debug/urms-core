use crate::core::graph::graph::Graph;

pub struct QueryEngine;

impl QueryEngine {
    pub fn find_node(graph: &Graph, name: &str) {
        for node in &graph.nodes {
            if node.name == name {
                println!("FOUND NODE:");
                println!("id => {}", node.id);
                println!("name => {}", node.name);
                return;
            }
        }

        println!("Node not found");
    }
}