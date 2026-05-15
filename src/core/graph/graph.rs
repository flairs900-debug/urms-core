use crate::core::graph::{
    node::Node,
    edge::Edge,
};

#[derive(Debug)]
pub struct Graph {

    pub nodes: Vec<Node>,

    pub edges: Vec<Edge>,
}

impl Graph {

    pub fn new() -> Self {

        Self {

            nodes: Vec::new(),

            edges: Vec::new(),
        }
    }

    pub fn add_node(
        &mut self,
        name: &str,
    ) {

        for node in &self.nodes {

            if node.name == name {

                println!(
                    "node already exists => {}",
                    name
                );

                return;
            }
        }

        let id =
            self.nodes.len() + 1;

        let node =
            Node::new(
                id,
                name.to_string(),
            );

        println!(
            "node added => {}",
            name
        );

        self.nodes.push(node);
    }

    pub fn add_edge(
        &mut self,
        from: usize,
        to: usize,
    ) {

        for edge in &self.edges {

            if edge.from == from
                && edge.to == to {

                println!(
                    "edge already exists => {} -> {}",
                    from,
                    to
                );

                return;
            }
        }

        let edge =
            Edge::new(
                from,
                to,
            );

        println!(
            "edge added => {} -> {}",
            from,
            to
        );

        self.edges.push(edge);
    }

    pub fn reinforce_node(
        &mut self,
        id: usize,
        amount: f32,
    ) {

        for node in &mut self.nodes {

            if node.id == id {

                node.reinforce(amount);

                println!(
                    "reinforced node => {}",
                    node.name
                );
            }
        }
    }

    pub fn decay(
        &mut self,
    ) {

        for node in &mut self.nodes {

            node.decay();
        }

        for edge in &mut self.edges {

            edge.decay();
        }

        println!(
            "graph decay applied"
        );
    }

    pub fn print_state(
        &self,
    ) {

        println!("GRAPH STATE");

        println!(
            "nodes => {}",
            self.nodes.len()
        );

        println!(
            "edges => {}",
            self.edges.len()
        );

        for node in &self.nodes {

            println!(
                "NODE [{}] {} activation={} weight={}",
                node.id,
                node.name,
                node.activation,
                node.weight,
            );
        }

        for edge in &self.edges {

            println!(
                "EDGE {} -> {} weight={}",
                edge.from,
                edge.to,
                edge.weight,
            );
        }
    }
}