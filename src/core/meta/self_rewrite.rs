use crate::core::graph::graph::Graph;
use crate::core::rewrite::engine::RewriteEngine;
use crate::core::rewrite::rule::RewriteRule;

pub fn self_rewrite(graph: &mut Graph) {
    let mut rewrite_engine = RewriteEngine::new();

    rewrite_engine.add_rule(
        RewriteRule::new(
            "ExecutionEngine",
            "AdaptiveExecutionEngine",
        )
    );

    rewrite_engine.execute(graph);
}