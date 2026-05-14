use std::fs;

use crate::core::graph::graph::Graph;
use crate::core::ontology::store::OntologyStore;

pub struct Persistence;

impl Persistence {

    pub fn save_graph(graph: &Graph) {

        let json =
            serde_json::to_string_pretty(graph)
                .unwrap();

        fs::write(
            "memory_graph.json",
            json
        )
        .unwrap();

        println!("Graph saved");
    }

    pub fn load_graph() -> Option<Graph> {

        let content =
            fs::read_to_string("memory_graph.json");

        match content {

            Ok(data) => {

                let graph: Graph =
                    serde_json::from_str(&data)
                        .unwrap();

                println!("Graph loaded");

                Some(graph)
            }

            Err(_) => None,
        }
    }

    pub fn save_ontology(
        ontology: &OntologyStore
    ) {

        let json =
            serde_json::to_string_pretty(ontology)
                .unwrap();

        fs::write(
            "memory_ontology.json",
            json
        )
        .unwrap();

        println!("Ontology saved");
    }

    pub fn load_ontology()
        -> Option<OntologyStore> {

        let content =
            fs::read_to_string(
                "memory_ontology.json"
            );

        match content {

            Ok(data) => {

                let ontology: OntologyStore =
                    serde_json::from_str(&data)
                        .unwrap();

                println!("Ontology loaded");

                Some(ontology)
            }

            Err(_) => None,
        }
    }
}