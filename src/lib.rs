pub mod core;
pub mod sim;
pub mod rewards;

#[cfg(test)]
mod tests {
    use super::sim::Node;

    #[test]
    fn create_node() {
        let node = Node::new(1, 0.5);
        assert_eq!(node.id, 1);
        assert_eq!(node.weight, 0.5);
    }
}