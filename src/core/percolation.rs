use std::collections::{HashMap, HashSet};

pub type Amount = i128;
pub const SCALE: Amount = 1_000_000; // 1.0 = 1_000_000 (micro-units)

pub fn units(x: f64) -> Amount {
    (x * (SCALE as f64)).round() as Amount
}

pub fn to_f64(a: Amount) -> f64 {
    (a as f64) / (SCALE as f64)
}

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

/// Adjacency edge (kept minimal for speed)
#[derive(Debug, Clone, Copy)]
struct AdjEdge {
    other: u64,   // for out_adj: this is "to"; for in_adj: this is "from"
    weight: f64,
}

#[derive(Debug, Default)]
pub struct Graph {
    pub nodes: Vec<u64>,
    pub edges: Vec<Edge>,

    // v0.7: adjacency indices
    out_adj: HashMap<u64, Vec<AdjEdge>>, // from -> [(to, w)]
    in_adj: HashMap<u64, Vec<AdjEdge>>,  // to   -> [(from, w)]
}

#[derive(Debug, Default)]
pub struct Ledger {
    pub balances: HashMap<u64, Amount>,
}

impl Ledger {
    pub fn total(&self) -> Amount {
        self.balances.values().copied().sum()
    }

    pub fn total_f64(&self) -> f64 {
        to_f64(self.total())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GraphError {
    InvalidWeight { from: u64, to: u64, weight: f64 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropagationStop {
    Stable,
    ReachedMaxIters,
}

#[derive(Debug, Clone)]
pub struct PropagationResult {
    pub active: HashSet<u64>,
    pub rounds: usize,
    pub stop: PropagationStop,
}

impl Graph {
    pub fn add_node(&mut self, id: u64) {
        if !self.nodes.contains(&id) {
            self.nodes.push(id);
            self.nodes.sort_unstable(); // deterministik
        }
    }

    /// v0.7: still Result (no panic), and also updates adjacency indices
    pub fn add_edge(&mut self, from: u64, to: u64, weight: f64) -> Result<(), GraphError> {
        if !weight.is_finite() || weight < 0.0 {
            return Err(GraphError::InvalidWeight { from, to, weight });
        }

        self.add_node(from);
        self.add_node(to);

        self.edges.push(Edge::new(from, to, weight));
        self.edges
            .sort_by(|a, b| (a.from, a.to).cmp(&(b.from, b.to))); // deterministik

        // update out adjacency
        {
            let v = self.out_adj.entry(from).or_insert_with(Vec::new);
            v.push(AdjEdge { other: to, weight });
            // deterministik adjacency order
            v.sort_by(|a, b| a.other.cmp(&b.other));
        }

        // update in adjacency
        {
            let v = self.in_adj.entry(to).or_insert_with(Vec::new);
            v.push(AdjEdge {
                other: from,
                weight,
            });
            v.sort_by(|a, b| a.other.cmp(&b.other));
        }

        Ok(())
    }

    pub fn outflow(&self, from: u64) -> f64 {
        match self.out_adj.get(&from) {
            None => 0.0,
            Some(v) => v.iter().map(|e| e.weight).sum(),
        }
    }

    pub fn inflow(&self, to: u64) -> f64 {
        match self.in_adj.get(&to) {
            None => 0.0,
            Some(v) => v.iter().map(|e| e.weight).sum(),
        }
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

    /// v0.6+: bounded iteration + status
    pub fn propagate_until_stable(
        &self,
        initial_active: HashSet<u64>,
        threshold: f64,
        max_iters: usize,
    ) -> PropagationResult {
        let mut current = initial_active;

        for round in 0..max_iters {
            let next = self.propagate_once(&current, threshold);
            if next == current {
                return PropagationResult {
                    active: current,
                    rounds: round,
                    stop: PropagationStop::Stable,
                };
            }
            current = next;
        }

        PropagationResult {
            active: current,
            rounds: max_iters,
            stop: PropagationStop::ReachedMaxIters,
        }
    }

    /// v0.7: reward distribution uses adjacency (fast) and fixed-point conservative allocation.
    pub fn distribute_rewards(&self, initial_rewards: &HashMap<u64, Amount>) -> Ledger {
        let mut ledger = Ledger::default();

        // deterministik: iterasi reward keys di-sort
        let mut reward_nodes: Vec<u64> = initial_rewards.keys().copied().collect();
        reward_nodes.sort_unstable();

        for node in reward_nodes {
            let amount = initial_rewards[&node];
            if amount <= 0 {
                continue;
            }

            let outgoing = match self.out_adj.get(&node) {
                None => {
                    *ledger.balances.entry(node).or_insert(0) += amount;
                    continue;
                }
                Some(v) if v.is_empty() => {
                    *ledger.balances.entry(node).or_insert(0) += amount;
                    continue;
                }
                Some(v) => v,
            };

            let total_weight: f64 = outgoing.iter().map(|e| e.weight).sum();
            if total_weight.abs() < 1e-12 {
                *ledger.balances.entry(node).or_insert(0) += amount;
                continue;
            }

            // floor allocation + remainder by largest fractional parts (deterministic)
            let mut floors: Vec<Amount> = Vec::with_capacity(outgoing.len());
            let mut fracs: Vec<(usize, f64)> = Vec::with_capacity(outgoing.len());

            let amount_f = amount as f64;
            let mut sum_floor: Amount = 0;

            for (i, e) in outgoing.iter().enumerate() {
                let ratio = e.weight / total_weight;
                let exact = amount_f * ratio;
                let floor_i = exact.floor() as Amount;
                let frac = exact - (floor_i as f64);

                floors.push(floor_i);
                fracs.push((i, frac));
                sum_floor += floor_i;
            }

            let mut remainder: Amount = amount - sum_floor;
            if remainder < 0 {
                remainder = 0;
            }

            // sort by frac desc; tie-breaker by destination (other) then index
            fracs.sort_by(|(ia, fa), (ib, fb)| {
                let c = fb.partial_cmp(fa).unwrap_or(std::cmp::Ordering::Equal);
                if c != std::cmp::Ordering::Equal {
                    return c;
                }
                let ta = outgoing[*ia].other;
                let tb = outgoing[*ib].other;
                ta.cmp(&tb).then_with(|| ia.cmp(ib))
            });

            let mut idx = 0usize;
            while remainder > 0 {
                let i = fracs[idx % fracs.len()].0;
                floors[i] += 1;
                remainder -= 1;
                idx += 1;
            }

            for (e, share) in outgoing.iter().zip(floors.into_iter()) {
                if share != 0 {
                    *ledger.balances.entry(e.other).or_insert(0) += share;
                }
            }
        }

        ledger
    }

    pub fn propagate_rewards_multi_round(
        &self,
        initial_rewards: HashMap<u64, Amount>,
        rounds: usize,
    ) -> Ledger {
        let mut current = initial_rewards;

        for _ in 0..rounds {
            let ledger = self.distribute_rewards(&current);
            current = ledger.balances;
        }

        Ledger { balances: current }
    }
}