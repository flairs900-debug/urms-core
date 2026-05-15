use crate::core::{
    graph::graph::Graph,

    observation::{
        engine::ObservationEngine,
        event::ObservationEvent,
    },

    interpretation::{
        engine::InterpretationEngine,
    },

    evolution::{
        engine::EvolutionEngine,
        history::EvolutionHistory,
    },

    memory::persistence::Persistence,
};

pub struct RuntimeLoop;

impl RuntimeLoop {

    pub fn run(graph: &mut Graph) {

        println!("runtime loop started");

        /*
            observation
        */

        let observation: ObservationEvent =
            ObservationEngine::observe(
                "system overload"
            );

        println!(
            "observation => {:?}",
            observation
        );

        /*
            interpretation
        */

        let interpreted =
            InterpretationEngine::interpret(
                &observation
            );

        println!(
            "interpreted => {:?}",
            interpreted
        );

        /*
            evolution
        */

        let mut history =
            EvolutionHistory::new();

        EvolutionEngine::evolve(
            graph,
            &mut history,
        );

        println!(
            "evolution applied"
        );

        /*
            persistence
        */

        Persistence::save_graph(
            graph
        );

        graph.print_state();

        println!("runtime loop finished");
    }
}