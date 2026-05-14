use crate::core::graph::graph::Graph;

use crate::core::memory::persistence::Persistence;

use crate::core::ontology::store::OntologyStore;

use crate::core::meta::evolution::MetaEvolution;

pub struct RuntimeLoop;

impl RuntimeLoop {

    pub fn start(
        graph: &mut Graph,
        ontology: &mut OntologyStore,
    ) {

        println!("Runtime loop started");

        MetaEvolution::mutate(graph);

        Persistence::save_graph(graph);

        Persistence::save_ontology(ontology);

        println!("SYSTEM OK");
    }
}