use super::relation::Relation;

#[derive(Debug, Clone)]

pub struct Edge {
    pub from: u64,
    pub to: u64,
    pub relation: Relation,
}