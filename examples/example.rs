use graph::{AdjacencyVecGraph, Graph};

#[derive(Debug)]
struct Node {
    name: String,
}

fn main() {
    let mut graph = AdjacencyVecGraph::<u8, Node>::new();
    println!("{graph:?}");
    graph.add_node(
        12,
        Node {
            name: "A".to_string(),
        },
    );
    graph.add_node(
        46,
        Node {
            name: "B".to_string(),
        },
    );
    graph.connect(12, 46);
    println!("{graph:?}");

    for node in graph.pre_order_dfs(&46) {
        println!("{node:?}");
        println!("name: {}", graph[*node].name);
    }
}
