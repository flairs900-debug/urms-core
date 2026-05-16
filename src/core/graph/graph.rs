#[derive(Clone, Debug)]
pub struct Node {
    pub id: usize,
    pub name: String,
    pub activation: f32,
    pub weight: f32,
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug)]
pub struct SemanticGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl SemanticGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, name: &str) -> usize {
        let id = self.nodes.len() + 1;

        self.nodes.push(Node {
            id,
            name: name.to_string(),
            activation: 0.0,
            weight: 1.0,
        });

        println!("created node -> {}", id);

        id
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push(Edge { from, to });

        println!("created edge -> {} -> {}", from, to);
    }
}