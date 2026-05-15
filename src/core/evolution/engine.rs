use crate::core::{
    evolution::history::{
        EvolutionHistory,
        EvolutionRecord,
    },
    graph::graph::Graph,
    rewrite::{
        engine::RewriteEngine,
        rule::RewriteRule,
    },
};

pub struct EvolutionEngine;

impl EvolutionEngine {
    pub fn evolve(
        graph: &mut Graph,
        history: &mut EvolutionHistory,
    ) {
        let rules = vec![
            RewriteRule::new(
                "ExecutionEngine",
                "AdaptiveExecutionEngine",
            ),
            RewriteRule::new(
                "SemanticEngine",
                "ReflectiveSemanticEngine",
            ),
        ];

        RewriteEngine::apply(
            graph,
            &rules,
        );

        for rule in rules {
            history.records.push(
                EvolutionRecord {
                    from: rule.from,
                    to: rule.to,
                },
            );
        }

        println!(
            "evolution applied"
        );
    }
}