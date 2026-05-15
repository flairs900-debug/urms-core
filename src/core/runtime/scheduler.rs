use crate::core::evolution::engine::EvolutionEngine;
use crate::core::evolution::history::EvolutionHistory;
use crate::core::graph::graph::Graph;
use crate::core::interpretation::engine::InterpretationEngine;
use crate::core::observation::engine::ObservationEngine;

pub struct Scheduler;

impl Scheduler {
    pub fn run(
        graph: &mut Graph,
        history: &mut EvolutionHistory,
    ) {

        let event =
            ObservationEngine::observe("fire");

        let interpreted =
            InterpretationEngine::interpret(&event);

        println!(
            "INTERPRET => {:?}",
            interpreted
        );

        EvolutionEngine::evolve(
            graph,
            history,
        );
    }
}