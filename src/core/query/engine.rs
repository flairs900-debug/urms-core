use crate::core::graph::{
    graph::Graph,
};

pub struct QueryEngine;

impl QueryEngine {

    pub fn ask_node(
        graph: &Graph,
        name: &str,
    ) {

        println!(
            "query node => {}",
            name
        );

        for node in &graph.nodes {

            if node.name == name {

                println!(
                    "FOUND NODE => id={} name={}",
                    node.id,
                    node.name
                );
            }
        }
    }

    pub fn ask_relations(
        graph: &Graph,
        node_id: usize,
    ) {

        println!(
            "query relations => {}",
            node_id
        );

        for edge in &graph.edges {

            if edge.from == node_id {

                println!(
                    "EDGE => {} -> {}",
                    edge.from,
                    edge.to
                );
            }
        }
    }

    pub fn show_graph(
        graph: &Graph,
    ) {

        graph.print_state();
    }
}