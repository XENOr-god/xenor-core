use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
}

#[derive(Debug)]
pub struct Graph {
    pub edges: HashMap<String, Vec<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        self.edges
            .entry(from.to_string())
            .or_insert_with(Vec::new)
            .push(to.to_string());
    }

    pub fn propagate(&self, start: &str) -> Vec<String> {
        let mut visited = Vec::new();

        if let Some(children) = self.edges.get(start) {
            for child in children {
                visited.push(child.clone());
            }
        }

        visited
    }
}
