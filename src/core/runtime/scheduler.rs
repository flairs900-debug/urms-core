use crate::core::{
    evolution::engine::EvolutionEngine,
    graph::graph::SemanticGraph,
    interpretation::InterpretationEngine,
};

pub struct RuntimeScheduler;

impl RuntimeScheduler {
    pub fn schedule(graph: &mut SemanticGraph) {
        println!("Scheduler started");

        InterpretationEngine::interpret("scheduler cycle");

        EvolutionEngine::evolve(graph);

        println!("Scheduler finished");
    }
}