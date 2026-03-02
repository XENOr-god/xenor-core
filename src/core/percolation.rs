use std::collections::{HashMap, HashSet};

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

#[derive(Debug, Default)]
pub struct Ledger {
    pub balances: HashMap<u64, f64>,
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

    pub fn propagate_once(&self, active: &HashSet<u64>, threshold: f64) -> HashSet<u64> {
        let mut next = active.clone();

        for &node in &self.nodes {
            if !active.contains(&node) && self.is_active(node, threshold) {
                next.insert(node);
            }
        }

        next
    }

    pub fn propagate_until_stable(
        &self,
        initial_active: HashSet<u64>,
        threshold: f64,
    ) -> HashSet<u64> {
        let mut current = initial_active;

        loop {
            let next = self.propagate_once(&current, threshold);
            if next == current {
                return current;
            }
            current = next;
        }
    }

    pub fn distribute_rewards(
        &self,
        initial_rewards: &HashMap<u64, f64>,
    ) -> Ledger {
        let mut ledger = Ledger::default();

        for (&node, &amount) in initial_rewards {
            if amount <= 0.0 {
                continue;
            }

            let outgoing: Vec<_> = self
                .edges
                .iter()
                .filter(|e| e.from == node)
                .collect();

            let total_weight: f64 = outgoing.iter().map(|e| e.weight).sum();

            if total_weight == 0.0 {
                *ledger.balances.entry(node).or_insert(0.0) += amount;
                continue;
            }

            for edge in outgoing {
                let share = amount * (edge.weight / total_weight);
                *ledger.balances.entry(edge.to).or_insert(0.0) += share;
            }
        }

        ledger
    }
}