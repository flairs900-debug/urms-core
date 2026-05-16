use std::fs;

use crate::core::graph::graph::SemanticGraph;

pub struct Persistence;

impl Persistence {
    pub fn save(graph: &SemanticGraph) {
        let data = format!("{:#?}", graph);

        fs::write("memory_dump.txt", data)
            .expect("Unable to save memory");
    }

    pub fn load() -> Option<SemanticGraph> {
        None
    }
}