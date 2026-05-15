mod core;

use crate::core::{
    graph::graph::Graph,

    runtime::event_loop::RuntimeLoop,

    memory::persistence::Persistence,

    query::engine::QueryEngine,
};

fn main() {

    println!("URMS Core starting...");

    /*
        graph
    */

    let mut graph =
        Graph::new();

    /*
        load graph
    */

    Persistence::load_graph(
        &mut graph
    );

    /*
        bootstrap
    */

    if graph.nodes.is_empty() {

        graph.add_node(
            "ExecutionEngine"
        );

        graph.add_node(
            "SemanticEngine"
        );

        graph.add_edge(
            1,
            2,
        );
    }

    /*
        runtime
    */

    RuntimeLoop::run(
        &mut graph
    );

    /*
        query
    */

    QueryEngine::show_graph(
        &graph
    );

    QueryEngine::ask_node(
        &graph,
        "ExecutionEngine"
    );

    QueryEngine::ask_relations(
        &graph,
        1
    );

    println!("URMS Core finished.");
}