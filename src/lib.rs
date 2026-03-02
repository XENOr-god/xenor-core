pub mod core;
pub mod sim;
pub mod rewards;

#[cfg(test)]
mod tests {
    use super::sim::Node;
    use super::core::percolation::Graph;
    use std::collections::HashSet;

    #[test]
    fn create_node() {
        let node = Node::new(1, 0.5);
        assert_eq!(node.id, 1);
        assert_eq!(node.weight, 0.5);
    }

    #[test]
    fn outflow_sums_edges() {
        let mut g = Graph::default();
        g.add_edge(1, 2, 0.4);
        g.add_edge(1, 3, 0.6);
        assert_eq!(g.outflow(1), 1.0);
    }

    #[test]
    fn node_activation_threshold() {
        let mut g = Graph::default();

        g.add_edge(1, 2, 0.3);
        g.add_edge(3, 2, 0.4);

        // total inflow to node 2 = 0.7
        assert_eq!(g.inflow(2), 0.7);
        assert!(g.is_active(2, 0.5));
        assert!(!g.is_active(2, 0.8));
    }

    #[test]
    fn iterative_propagation() {
        let mut g = Graph::default();

        g.add_edge(1, 2, 0.6);
        g.add_edge(2, 3, 0.6);

        let mut initial = HashSet::new();
        initial.insert(1);

        let result = g.propagate_until_stable(initial, 0.5);

        assert!(result.contains(&1));
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }
}