use std::fs::File;

use std::io::Write;

use crate::core::graph::graph::Graph;

pub struct PersistenceEngine;

impl PersistenceEngine {

    pub fn save(graph: &Graph) {

        println!("Persistence engine active");

        let mut file = File::create(
            "memory_dump.txt"
        )
        .expect("Cannot create file");

        for node in &graph.nodes {

            let line = format!(
                "{}\n",
                node.name
            );

            file.write_all(
                line.as_bytes()
            )
            .expect("Write failed");
        }

        println!("Graph saved to memory_dump.txt");
    }
}