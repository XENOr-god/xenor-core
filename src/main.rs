mod propagation;

use propagation::Graph;

fn main() {
    let mut graph = Graph::new();

    graph.add_edge("Protocol", "NodeA");
    graph.add_edge("NodeA", "NodeB");
    graph.add_edge("NodeA", "NodeC");

    let result = graph.propagate("NodeA");

    println!("XENØr core engine starting");
    println!("Propagation from NodeA -> {:?}", result);
}
