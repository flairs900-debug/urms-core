use crate::core::graph::graph::SemanticGraph;

pub struct PersistentMemory;

impl PersistentMemory {

    pub fn save(graph: &SemanticGraph) {

        println!("PERSISTENT MEMORY SAVE");

        println!(
            "saved nodes={} edges={}",
            graph.nodes.len(),
            graph.edges.len()
        );
    }

    pub fn load() {

        println!("PERSISTENT MEMORY LOAD");
    }

    pub fn snapshot() {

        println!("MEMORY SNAPSHOT CREATED");
    }

    pub fn rollback() {

        println!("ROLLBACK EXECUTED");
    }
}