pub mod core;
pub mod sim;
pub mod rewards;

#[cfg(test)]
mod tests {
    use super::sim::Node;
    use super::core::percolation::Graph;

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
}