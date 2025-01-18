# Graph

## Description

This is a small library that provides both a `Graph trait` and an implementation `AdjacencyVecGraph`.

### `Graph trait`

The `Graph trait` can be used to treat a struct as a `Graph`, a graph has to specify two `type`s: `NodeIdType` and `NodeType`, because a `Graph` has to be treated sort of like a `Map` with some special relationship within the `key`s. Other than the type, a `Graph` implementation has to specify an iterator over all the `NodeId`s of the nodes, `iter`, and another iterator of all the `NodeId`s adjacent to a given `NodeId`.

#### Other methods

If the `NodeId` type implements the traits `Hash` and `Eq`, a list of further methods become available:

##### `pre_order_dfs`

This method takes a root `NodeId` and performs a Pre-Order Depth First Search starting from the root node, giving a lazy iterator over the `NodeId`s of the `Graph`.

```rust
println!("Pre-Order Depth First Search:");
let graph = construct_graph();
let root = graph.get_root();
for id in graph.pre_order_dfs(root) {
    println!("{}", graph[id]);
}
```
  
##### `post_order_dfs`

This method takes a root `NodeId` and performs a Post-Order Depth First Search starting from the root node, giving a lazy iterator over the `NodeId`s of the `Graph`.

```rust
println!("Post-Order Depth First Search:");
let graph = construct_graph();
let root = graph.get_root();
for id in graph.post_order_dfs(root) {
    println!("{}", graph[id]);
}
```

##### `bfs`

This method takes a root `NodeId` and performs a Breadth First Search starting from the root node, giving a lazy iterator over the `NodeId`s of the `Graph`.

```rust
println!("Breadth First Search:");
let graph = construct_graph();
let root = graph.get_root();
for id in graph.bfs(root) {
    println!("{}", graph[id]);
}
```

##### `scc` (to-do)

This method finds all the strongly connected components of the `Graph`, it returns an iterator of the strongly connected components of the `Graph` with every strongly connected component being just a further iterator of `NodeId`s.

```rust
println!("Strongly Connected Components:");
let graph = construct_graph();
for (index, iterator) in graph.scc().enumerate() {
    println!("#{}", index);
    for id in iterator {
        println!("{}", graph[id]);
    }
}
```

##### `top_sort` (to-do)

This method performs topological sorting on the `Graph` giving an iterator over the `NodeId`s such that if a `NodeId` is after another one it means that there's not a path going from the latter to the former

```rust
println!("Topological sorting:");
let graph = construct_graph();
for id in graph.top_sort() {
    println!("{}", graph[id]);
}
```

### `AdjacencyVecGraph`

This kind of `Graph` stores the information in a `HashMap<NodeId, (NodeType, HashSet<NodeId>)>` to make every operation as fast as possible.

## Usage

In order to import this library just add the following line into your `Cargo.toml` file:

```toml
[dependencies]
graph = { git = "https://github.com/daw-dev/graph" }
```

After adding the dependency run `cargo update` to update the library to the newer version.

Then, you can just import it in your `.rs` files (taken from `examples/example.rs`):

```rust
use graph::{AdjacencyVecGraph, Graph};

#[derive(Debug)]
struct Node {
    name: String,
}

pub fn main() {
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
    }
}
```