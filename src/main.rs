mod core;

use core::graph::graph::Graph;
use core::graph::node::Node;

use core::rewrite::engine::RewriteEngine;
use core::rewrite::rule::RewriteRule;

use core::runtime::engine::RuntimeEngine;
use core::runtime::r#loop::RuntimeLoop;

use core::query::engine::QueryEngine;

use core::ontology::store::OntologyStore;

use core::memory::persistence::Persistence;

fn main() {

    println!("URMS CORE START");

    RuntimeEngine::run();

    // =========================
    // LOAD GRAPH
    // =========================

    let mut graph =
        Persistence::load_graph()
            .unwrap_or(Graph::new());

    // =========================
    // DEFAULT GRAPH
    // =========================

    if graph.nodes.is_empty() {

        graph.add_node(Node {
            id: 1,
            name: "ExecutionEngine".to_string(),
        });
    }

    // =========================
    // REWRITE ENGINE
    // =========================

    let rule = RewriteRule {
        from: "ExecutionEngine".to_string(),
        to: "AdaptiveExecutionEngine".to_string(),
    };

    let mut rewrite_engine = RewriteEngine::new();

rewrite_engine.add_rule(
    RewriteRule {
        from: "ExecutionEngine".to_string(),
        to: "AdaptiveExecutionEngine".to_string(),
    }
);

rewrite_engine.execute(&mut graph);

    // =========================
    // QUERY ENGINE
    // =========================

    QueryEngine::find_node(
        &graph,
        "AdaptiveExecutionEngine"
    );

    // =========================
    // ONTOLOGY
    // =========================

    let mut ontology =
        Persistence::load_ontology()
            .unwrap_or(OntologyStore::new());

    if ontology.entities.is_empty() {

        ontology.add_entity(
            "Cognition",
            "concept"
        );

        ontology.add_entity(
            "LongTermMemory",
            "memory"
        );

        ontology.add_entity(
            "RuntimeKernel",
            "runtime"
        );
    }

    println!("=== ONTOLOGY ===");

    for entity in &ontology.entities {

        println!(
            "{} ({})",
            entity.name,
            entity.kind
        );
    }

    // =========================
    // SAVE
    // =========================

    graph.save("memory_dump.json");

    ontology.save("ontology_dump.json");

    // =========================
    // RUNTIME LOOP
    // =========================

    RuntimeLoop::start(
        &mut graph,
        &mut ontology
    );

    println!("SYSTEM OK");
}