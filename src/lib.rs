pub mod core;
pub mod rewards;
pub mod sim;

#[cfg(test)]
mod tests {
    use super::core::percolation::{Amount, Graph, PropagationStop, SCALE, units};
    use super::sim::Node;
    use std::collections::{HashMap, HashSet};

    fn assert_amount_eq(a: Amount, b: Amount) {
        assert_eq!(a, b);
    }

    #[test]
    fn create_node() {
        let node = Node::new(1, 0.5);
        assert_eq!(node.id, 1);
        assert_eq!(node.weight, 0.5);
    }

    #[test]
    fn outflow_sums_edges() {
        let mut g = Graph::default();
        g.add_edge(1, 2, 0.4).unwrap();
        g.add_edge(1, 3, 0.6).unwrap();
        assert!((g.outflow(1) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn node_activation_threshold() {
        let mut g = Graph::default();

        g.add_edge(1, 2, 0.3).unwrap();
        g.add_edge(3, 2, 0.4).unwrap();

        assert!((g.inflow(2) - 0.7).abs() < 1e-12);
        assert!(g.is_active(2, 0.5));
        assert!(!g.is_active(2, 0.8));
    }

    #[test]
    fn iterative_propagation() {
        let mut g = Graph::default();

        g.add_edge(1, 2, 0.6).unwrap();
        g.add_edge(2, 3, 0.6).unwrap();

        let mut initial = HashSet::new();
        initial.insert(1);

        let res = g.propagate_until_stable(initial, 0.5, 100);
        assert_eq!(res.stop, PropagationStop::Stable);

        let result = res.active;
        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }

    #[test]
    fn reward_distribution() {
        let mut g = Graph::default();

        g.add_edge(1, 2, 0.6).unwrap();
        g.add_edge(1, 3, 0.4).unwrap();

        let mut rewards = HashMap::new();
        rewards.insert(1, units(100.0));

        let ledger = g.distribute_rewards(&rewards);

        assert_amount_eq(*ledger.balances.get(&2).unwrap(), units(60.0));
        assert_amount_eq(*ledger.balances.get(&3).unwrap(), units(40.0));
        assert_amount_eq(ledger.total(), units(100.0));
    }

    #[test]
    fn multi_round_propagation_and_conservation() {
        let mut g = Graph::default();

        g.add_edge(1, 2, 1.0).unwrap();
        g.add_edge(2, 3, 1.0).unwrap();

        let mut rewards = HashMap::new();
        rewards.insert(1, units(100.0));

        let ledger = g.propagate_rewards_multi_round(rewards.clone(), 2);

        let initial_total: Amount = rewards.values().copied().sum();
        let final_total = ledger.total();
        assert_amount_eq(initial_total, final_total);

        // sanity: should end up at node 3 after 2 rounds
        assert_amount_eq(*ledger.balances.get(&3).unwrap(), 100 * SCALE);
    }
}
