#[derive(Debug, Clone)]
pub struct Edge {
    pub from: u64,
    pub to: u64,
    pub weight: f64,
}

impl Edge {
    pub fn new(from: u64, to: u64, weight: f64) -> Self {
        Self { from, to, weight }
    }
}

#[derive(Debug, Default)]
pub struct Graph {
    pub nodes: Vec<u64>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn add_node(&mut self, id: u64) {
        if !self.nodes.contains(&id) {
            self.nodes.push(id);
        }
    }

    pub fn add_edge(&mut self, from: u64, to: u64, weight: f64) {
        self.add_node(from);
        self.add_node(to);
        self.edges.push(Edge::new(from, to, weight));
    }

    pub fn outflow(&self, from: u64) -> f64 {
        self.edges
            .iter()
            .filter(|e| e.from == from)
            .map(|e| e.weight)
            .sum()
    }

    pub fn inflow(&self, to: u64) -> f64 {
        self.edges
            .iter()
            .filter(|e| e.to == to)
            .map(|e| e.weight)
            .sum()
    }

    pub fn is_active(&self, node: u64, threshold: f64) -> bool {
        self.inflow(node) >= threshold
    }
}