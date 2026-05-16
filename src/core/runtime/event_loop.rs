use crate::core::graph::graph::SemanticGraph;

pub fn process_events(graph: &mut SemanticGraph) {

    println!("event loop started");

    graph.add_node("event");

    graph.add_node("signal");

    graph.add_edge(1, 2);

    println!("event loop finished");
}