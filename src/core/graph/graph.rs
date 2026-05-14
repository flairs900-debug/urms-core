use serde::{Serialize, Deserialize};

use std::fs::File;
use std::io::Write;

use super::node::Node;
use super::edge::Edge;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {

    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn save(&self, path: &str) {

        let json =
            serde_json::to_string_pretty(self)
                .expect("Failed to serialize graph");

        let mut file =
            File::create(path)
                .expect("Failed to create graph file");

        file.write_all(json.as_bytes())
            .expect("Failed to write graph");
    }
}