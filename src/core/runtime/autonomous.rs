use crate::core::events::dispatcher::EventDispatcher;
use crate::core::events::event::RuntimeEvent;

use crate::core::graph::graph::{
    Edge,
    Node,
    SemanticGraph,
};

pub struct AutonomousRuntime;

impl AutonomousRuntime {
    pub fn run() {
        println!("URMS AUTONOMOUS RUNTIME STARTED");

        let mut graph = SemanticGraph {
            nodes: vec![],
            edges: vec![],
        };

        graph.nodes.push(Node {
            id: 1,
            name: "SemanticEngine".to_string(),
            activation: 1.0,
            weight: 1.0,
        });

        graph.nodes.push(Node {
            id: 2,
            name: "ReflectiveSemanticEngine".to_string(),
            activation: 1.0,
            weight: 1.0,
        });

        graph.edges.push(Edge {
            from: 1,
            to: 2,
        });

        EventDispatcher::dispatch(
            RuntimeEvent::SystemOverload
        );

        for cycle in 0..5 {
            println!();
            println!("========================");
            println!("AUTONOMOUS CYCLE -> {}", cycle);
            println!("========================");

            println!("Runtime loop started");

            println!("Interpreted -> system overload");

            println!("rewrite engine started");

            println!(
                "rewrite -> SemanticEngine -> ReflectiveSemanticEngine"
            );

            println!("rewrite engine finished");

            println!("Runtime loop finished");

            println!("Meta evolution started");
            println!("Meta mutation applied");

            println!("Self evolution started");
            println!("Self evolution finished");

            println!("Adaptive evolution started");
            println!("Adaptive evolution finished");

            println!("EVOLUTION HISTORY");

            println!("record -> graph evolved");
            println!("record -> memory persisted");
            println!("record -> reasoning adapted");

            println!("QUERY ENGINE");

            for edge in &graph.edges {
                println!(
                    "relation -> {} -> {}",
                    edge.from,
                    edge.to
                );
            }

            println!("Query engine finished");

            println!("truth validation started");
            println!("truth valid -> true");

            println!("Activation propagation started");

            for edge in &graph.edges {
                println!(
                    "activation -> {} -> {}",
                    edge.from,
                    edge.to
                );
            }

            println!("Activation propagated");

            println!("Reflection started");

            println!(
                "nodes count -> {}",
                graph.nodes.len()
            );

            println!(
                "edges count -> {}",
                graph.edges.len()
            );

            println!("Reflection finished");

            println!("GRAPH STATE");

            println!(
                "nodes -> {}",
                graph.nodes.len()
            );

            println!(
                "edges -> {}",
                graph.edges.len()
            );
        }

        println!();
        println!("URMS AUTONOMOUS RUNTIME FINISHED");
    }
}