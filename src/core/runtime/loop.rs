use crate::core::{
    evolution::{
        engine::EvolutionEngine,
        history::EvolutionHistory,
    },
    graph::graph::Graph,
    interpretation::engine::InterpretationEngine,
    memory::persistence::Persistence,
    observation::engine::ObservationEngine,
    ontology::store::OntologyStore,
    reflection::analyzer::ReflectionAnalyzer,
};

pub struct RuntimeLoop;

impl RuntimeLoop {
    pub fn run(
        graph: &mut Graph,
        ontology: &OntologyStore,
    ) {
        println!(
            "runtime loop started"
        );

        let observation =
            ObservationEngine::observe(
                "system overload",
            );

        let interpreted =
            InterpretationEngine::interpret(
                &observation,
            );

        println!(
            "interpreted => {}",
            interpreted.value
        );

        let mut history =
            EvolutionHistory::new();

        EvolutionEngine::evolve(
            graph,
            &mut history,
        );

        ReflectionAnalyzer::analyze(
            graph,
        );

        Persistence::save_graph(
    graph
);

        println!(
            "ontology entities => {}",
            ontology.entities.len()
        );

        println!(
            "runtime loop finished"
        );
    }
}