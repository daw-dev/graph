use std::{
    collections::{HashMap, HashSet}, fmt::Debug, hash::Hash, marker::PhantomData, ops::{Index, IndexMut}
};

use super::graph::Graph;

pub struct AdjacencyVecGraph<'a, NodeId, NodeType>
where
    NodeId: Hash + Eq,
{
    matrix: HashMap<NodeId, (NodeType, HashSet<NodeId>)>,
    phantom: PhantomData<&'a NodeId>,
}

impl<'a, NodeId, Node> AdjacencyVecGraph<'a, NodeId, Node>
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

    pub fn get(&self, node_id: &NodeId) -> Option<&Node> {
        self.matrix.get(node_id).map(|(info, _)| info)
    }

    pub fn get_mut(&mut self, node_id: &NodeId) -> Option<&mut Node> {
        self.matrix.get_mut(node_id).map(|(info, _)| info)
    }
}

impl<'a, Id, Node> Graph<'a> for AdjacencyVecGraph<'a, Id, Node>
where
    Id: Hash + Eq,
{
    type NodeId = &'a Id;

    fn adjacents(&'a self, node: &'a Id) -> impl Iterator<Item = &'a Id> {
        self.matrix[node].1.iter()
    }

    fn iter(&'a self) -> impl Iterator<Item = &'a Id> {
        self.matrix.keys()
    }
}

impl<NodeId, Node> Debug for AdjacencyVecGraph<'_, NodeId, Node>
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

impl<NodeId, Node> Default for AdjacencyVecGraph<'_, NodeId, Node>
where
    NodeId: Hash + Eq,
{
    fn default() -> Self {
        Self {
            matrix: Default::default(),
            phantom: PhantomData,
        }
    }
}

impl<NodeId, Node> Index<NodeId> for AdjacencyVecGraph<'_, NodeId, Node>
where
    NodeId: Hash + Eq,
{
    type Output = Node;

    fn index(&self, index: NodeId) -> &Self::Output {
        self.get(&index).expect("Node not found")
    }
}

impl<NodeId, Node> IndexMut<NodeId> for AdjacencyVecGraph<'_, NodeId, Node>
where
    NodeId: Hash + Eq,
{
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        self.get_mut(&index).expect("Node not found")
    }
}
