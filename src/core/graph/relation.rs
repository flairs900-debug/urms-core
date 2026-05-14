#[derive(Debug, Clone)]

pub enum Relation {

    Controls,

    DependsOn,

    Contains,

    Executes,

    EvolvesTo,
}