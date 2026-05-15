use crate::core::{
    graph::graph::Graph,
    rewrite::rule::RewriteRule,
};

pub struct RewriteEngine;

impl RewriteEngine {
    pub fn apply(
        graph: &mut Graph,
        rules: &Vec<RewriteRule>,
    ) {
        println!(
            "rewrite engine started"
        );

        for rule in rules {
            for node in &mut graph.nodes {
                if node.name == rule.from {
                    println!(
                        "rewrite => {} -> {}",
                        node.name,
                        rule.to
                    );

                    node.name =
                        rule.to.clone();
                }
            }
        }

        println!(
            "rewrite engine finished"
        );
    }
}