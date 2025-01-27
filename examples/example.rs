use graph::{AdjacencyVecGraph, ReferenceGraph};

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

    let mut graph = [[false; 3]; 3];
    graph[0][1] = true;
    graph[1][2] = true;
    graph[2][0] = true;
    //    0  1  2
    // 0  0  1  0
    // 1  0  0  1
    // 2  1  0  0
    // for node in graph.bfs(0) {
    //     println!("{node}");
    // }
}
