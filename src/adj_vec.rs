use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use super::graph::Graph;

pub struct AdjacencyVecGraph<NodeId, NodeType>
where
    NodeId: Hash + Eq,
{
    matrix: HashMap<NodeId, (NodeType, HashSet<NodeId>)>,
}

impl<NodeId, Node> AdjacencyVecGraph<NodeId, Node>
where
    NodeId: Hash + Eq,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_node(&mut self, node_id: NodeId, node_info: Node) {
        self.matrix.insert(node_id, (node_info, HashSet::new()));
    }

    pub fn connect(&mut self, from: NodeId, to: NodeId) {
        if from == to {
            return;
        }

        if let Some((_, adjacents)) = self.matrix.get_mut(&from) {
            adjacents.insert(to);
        }
    }
}

impl<NodeId, Node> Graph for AdjacencyVecGraph<NodeId, Node>
where
    NodeId: Hash + Eq,
{
    type NodeId = NodeId;

    fn adjacents(&self, node: &Self::NodeId) -> impl Iterator<Item = &Self::NodeId> {
        self.matrix[node].1.iter()
    }

    fn iter(&self) -> impl Iterator<Item = &Self::NodeId> {
        self.matrix.keys()
    }
}

impl<NodeId, Node> Debug for AdjacencyVecGraph<NodeId, Node>
where
    NodeId: Hash + Eq + Debug,
    Node: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "AdjacencyVecGraph {{")?;
        for (id, (node, adjacents)) in self.matrix.iter() {
            writeln!(f, "  Node {id:?} {{")?;
            writeln!(f, "    data: {node:?},")?;
            writeln!(f, "    adjacents: [ {} ],", adjacents.iter().map(|id| format!("{id:?}")).collect::<Vec<_>>().join(", "))?;
            writeln!(f, "  }},")?;
        }
        write!(f, "}}")?;

        Ok(())
    }
}

impl<NodeId, Node> Default for AdjacencyVecGraph<NodeId, Node>
where
    NodeId: Hash + Eq,
{
    fn default() -> Self {
        Self {
            matrix: Default::default(),
        }
    }
}
