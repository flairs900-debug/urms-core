use std::fs;

use serde::{
    Deserialize,
    Serialize,
};

use crate::core::graph::{
    edge::Edge,
    graph::Graph,
    node::Node,
};

pub struct Persistence;

#[derive(
    Serialize,
    Deserialize,
)]
struct SavedNode {

    id: u64,
    name: String,
}

#[derive(
    Serialize,
    Deserialize,
)]
struct SavedEdge {

    from: u64,
    to: u64,
}

#[derive(
    Serialize,
    Deserialize,
)]
struct SavedGraph {

    nodes: Vec<SavedNode>,
    edges: Vec<SavedEdge>,
}

impl Persistence {

    pub fn save_graph(
        graph: &Graph,
    ) {

        let saved = SavedGraph {

            nodes: graph
                .nodes
                .iter()
                .map(|n| {

                    SavedNode {

                        id: n.id as u64,

                        name: n.name.clone(),
                    }

                })
                .collect(),

            edges: graph
                .edges
                .iter()
                .map(|e| {

                    SavedEdge {

                        from: e.from as u64,

                        to: e.to as u64,
                    }

                })
                .collect(),
        };

        let json =
            serde_json::to_string_pretty(
                &saved
            )
            .unwrap();

        fs::write(
            "runtime_graph.txt",
            json,
        )
        .unwrap();

        println!(
            "graph saved => runtime_graph.txt"
        );
    }

    pub fn load_graph(
        graph: &mut Graph,
    ) {

        let content =
            fs::read_to_string(
                "runtime_graph.txt"
            );

        if content.is_err() {

            println!(
                "no saved graph found"
            );

            return;
        }

        let json =
            content.unwrap();

        let parsed =
            serde_json::from_str::<SavedGraph>(
                &json
            );

        if parsed.is_err() {

            println!(
                "invalid graph file"
            );

            return;
        }

        let parsed =
            parsed.unwrap();

        graph.nodes.clear();

        graph.edges.clear();

        for n in parsed.nodes {

            graph.nodes.push(

                Node {

                    id: n.id as usize,

                    name: n.name,

                    activation: 0.0,

                    created_at: 0,

                    updated_at: 0,

                    weight: 1.0,
                }
            );
        }

        for e in parsed.edges {

            let from =
                e.from as usize;

            let to =
                e.to as usize;

            if from >= graph.nodes.len() {

                continue;
            }

            if to >= graph.nodes.len() {

                continue;
            }

            graph.edges.push(

                Edge {

                    from,
                    to,

                    weight: 1.0,
                }
            );
        }

        println!(
            "graph loaded"
        );
    }
}