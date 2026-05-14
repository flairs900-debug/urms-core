mod core;

use core::graph::graph::Graph;
use core::graph::node::Node;

use core::rewrite::engine::RewriteEngine;
use core::rewrite::rule::RewriteRule;

use core::runtime::engine::RuntimeEngine;
use core::query::engine::QueryEngine;

use core::ontology::store::OntologyStore;
use core::ontology::entity::Entity;

fn main() {
    println!("URMS CORE START");

    RuntimeEngine::run();

    let mut graph = Graph::new();

    graph.add_node(Node {
        id: 1,
        name: "ExecutionEngine".to_string(),
    });

    graph.add_node(Node {
        id: 2,
        name: "MemoryNode".to_string(),
    });

    QueryEngine::find_node(
        &graph,
        "ExecutionEngine",
    );

    RewriteEngine::rewrite(
        &mut graph,
        &RewriteRule {
            from: "ExecutionEngine".to_string(),
            to: "AdaptiveExecutionEngine".to_string(),
        },
    );

    QueryEngine::find_node(
        &graph,
        "AdaptiveExecutionEngine",
    );

    let mut ontology = OntologyStore::new();

    ontology.add_entity(
        Entity::new(
            "Cognition",
            "Concept",
        )
    );

    println!("SYSTEM OK");
}