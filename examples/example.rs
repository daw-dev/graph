use graph::{AdjacencyVecGraph, ReferenceGraph};

#[derive(Debug, Clone)]
struct Node {
    name: String,
}

fn main() {
    let graph: AdjacencyVecGraph<i32, Node> = AdjacencyVecGraph::from_iter([
        (
            1,
            (
                Node {
                    name: "Node 1".to_string(),
                },
                vec![2],
            ),
        ),
        (
            2,
            (
                Node {
                    name: "Node 2".to_string(),
                },
                vec![1, 3, 4],
            ),
        ),
        (
            3,
            (
                Node {
                    name: "Node 3".to_string(),
                },
                vec![2, 4],
            ),
        ),
        (
            4,
            (
                Node {
                    name: "Node 4".to_string(),
                },
                vec![2, 3],
            ),
        ),
    ]);

    println!("{:?}", graph);

    println!("Is connected? {}", graph.is_connected_undirected());

    let root = graph.keys().next().unwrap();
    let dfs = graph.pre_order_dfs(root);
    for node in dfs {
        println!("{:?}", node);
    }
}
