use crate::core::graph::graph::Graph;
use crate::core::rewrite::rule::RewriteRule;

pub struct RewriteEngine {
    rules: Vec<RewriteRule>,
}

impl RewriteEngine {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: RewriteRule) {
        self.rules.push(rule);
    }

    pub fn execute(&self, graph: &mut Graph) {
        for rule in &self.rules {
            Self::rewrite(graph, rule);
        }
    }

    pub fn rewrite(graph: &mut Graph, rule: &RewriteRule) {
        for node in &mut graph.nodes {
            if node.name == rule.from {
                node.name = rule.to.clone();
            }
        }
    }
}