use super::graph::ReferenceGraph;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
    ops::{Index, IndexMut},
};

#[derive(Clone)]
pub struct AdjacencyVecGraph<NodeKey, NodeValue = NodeKey>
where
    NodeKey: Hash + Eq,
{
    matrix: HashMap<NodeKey, (NodeValue, HashSet<NodeKey>)>,
}

impl<NodeKey, NodeValue> AdjacencyVecGraph<NodeKey, NodeValue>
where
    NodeKey: Hash + Eq,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            matrix: HashMap::with_capacity(capacity),
        }
    }

    pub fn add_node(&mut self, node_id: NodeKey, node_info: NodeValue) {
        self.matrix.insert(node_id, (node_info, HashSet::new()));
    }

    pub fn add_node_with_adjacent_capacity(
        &mut self,
        node_id: NodeKey,
        node_info: NodeValue,
        capacity: usize,
    ) {
        self.matrix
            .insert(node_id, (node_info, HashSet::with_capacity(capacity)));
    }

    pub fn is_adjacent_to(&self, from: &NodeKey, to: &NodeKey) -> bool {
        self.matrix
            .get(from)
            .map_or(false, |(_, adjacents)| adjacents.contains(to))
    }

    pub fn add_directed_edge(&mut self, from: NodeKey, to: NodeKey) {
        if from == to {
            return;
        }

        if let Some((_, adjacents)) = self.matrix.get_mut(&from) {
            adjacents.insert(to);
        }
    }

    pub fn add_undirected_edge(&mut self, from: NodeKey, to: NodeKey)
    where
        NodeKey: Clone,
    {
        self.add_directed_edge(from.clone(), to.clone());
        self.add_directed_edge(to, from);
    }

    pub fn get(&self, node_id: &NodeKey) -> Option<&NodeValue> {
        self.matrix.get(node_id).map(|(info, _)| info)
    }

    pub fn get_mut(&mut self, node_id: &NodeKey) -> Option<&mut NodeValue> {
        self.matrix.get_mut(node_id).map(|(info, _)| info)
    }

    pub fn remove_node(&mut self, node_id: &NodeKey) -> Option<NodeValue> {
        for (_, adjacents) in self.matrix.values_mut() {
            adjacents.remove(node_id);
        }

        self.matrix.remove(node_id).map(|(info, _)| info)
    }

    pub fn remove_directed_edge(&mut self, from: &NodeKey, to: &NodeKey) {
        if let Some((_, adjacents)) = self.matrix.get_mut(from) {
            adjacents.remove(to);
        }
    }

    pub fn remove_undirected_edge(&mut self, from: &NodeKey, to: &NodeKey) {
        self.remove_directed_edge(from, to);
        self.remove_directed_edge(to, from);
    }

    pub fn clear(&mut self) {
        self.matrix.clear();
    }

    pub fn node_count(&self) -> usize {
        self.matrix.len()
    }

    pub fn edge_count(&self) -> usize {
        self.matrix
            .values()
            .map(|(_, adjacents)| adjacents.len())
            .sum()
    }

    pub fn is_empty(&self) -> bool {
        self.matrix.is_empty()
    }

    pub fn contains_node(&self, node_id: &NodeKey) -> bool {
        self.matrix.contains_key(node_id)
    }

    pub fn contains_edge(&self, from: &NodeKey, to: &NodeKey) -> bool {
        self.matrix
            .get(from)
            .map_or(false, |(_, adjacents)| adjacents.contains(to))
    }

    pub fn drain(&mut self) -> impl Iterator<Item = (NodeKey, NodeValue)> + '_ {
        self.matrix.drain().map(|(id, (node, _))| (id, node))
    }

    pub fn values(&self) -> impl Iterator<Item = &NodeValue> {
        self.matrix.values().map(|(node, _)| node)
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut NodeValue> {
        self.matrix.values_mut().map(|(node, _)| node)
    }

    pub fn into_values(self) -> impl Iterator<Item = NodeValue> {
        self.matrix.into_iter().map(|(_, (node, _))| node)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&NodeKey, &NodeValue)> {
        self.matrix.iter().map(|(id, (node, _))| (id, node))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&NodeKey, &mut NodeValue)> {
        self.matrix.iter_mut().map(|(id, (node, _))| (id, node))
    }

    pub fn into_iter(self) -> impl Iterator<Item = (NodeKey, NodeValue)> {
        self.matrix.into_iter().map(|(id, (node, _))| (id, node))
    }

    pub fn edges(&self) -> impl Iterator<Item = (&NodeKey, &NodeKey)> {
        self.matrix
            .iter()
            .flat_map(|(from, (_, adjacents))| adjacents.iter().map(move |to| (from, to)))
    }

    pub fn map_values<OtherNodeValue>(
        self,
        mut f: impl FnMut(NodeValue) -> OtherNodeValue,
    ) -> AdjacencyVecGraph<NodeKey, OtherNodeValue> {
        let matrix = self
            .matrix
            .into_iter()
            .map(|(id, (node, adjacents))| (id, (f(node), adjacents)))
            .collect();

        AdjacencyVecGraph { matrix }
    }

    pub fn map_values_with_key<OtherNodeValue>(
        self,
        mut f: impl FnMut(&NodeKey, NodeValue) -> OtherNodeValue,
    ) -> AdjacencyVecGraph<NodeKey, OtherNodeValue> {
        let matrix = self
            .matrix
            .into_iter()
            .map(|(id, (node, adjacents))| {
                let new_node = f(&id, node);
                (id, (new_node, adjacents))
            })
            .collect();

        AdjacencyVecGraph { matrix }
    }
}

impl<NodeKey, NodeValue> ReferenceGraph for AdjacencyVecGraph<NodeKey, NodeValue>
where
    NodeKey: Hash + Eq,
{
    type NodeKey = NodeKey;

    fn adjacents(&self, node: &Self::NodeKey) -> impl Iterator<Item = &Self::NodeKey> {
        self.matrix[node].1.iter()
    }

    fn keys(&self) -> impl Iterator<Item = &Self::NodeKey> {
        self.matrix.keys()
    }
}

impl<NodeKey, NodeValue> Debug for AdjacencyVecGraph<NodeKey, NodeValue>
where
    NodeKey: Hash + Eq + Debug,
    NodeValue: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "AdjacencyVecGraph {{")?;
        for (id, (node, adjacents)) in self.matrix.iter() {
            writeln!(f, "  Node {id:?} {{")?;
            writeln!(f, "    data: {node:?},")?;
            writeln!(
                f,
                "    adjacents: [ {} ],",
                adjacents
                    .iter()
                    .map(|id| format!("{id:?}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
            writeln!(f, "  }},")?;
        }
        write!(f, "}}")?;

        Ok(())
    }
}

impl<NodeKey, NodeValue> Default for AdjacencyVecGraph<NodeKey, NodeValue>
where
    NodeKey: Hash + Eq,
{
    fn default() -> Self {
        Self {
            matrix: Default::default(),
        }
    }
}

impl<NodeKey, NodeValue> Index<NodeKey> for AdjacencyVecGraph<NodeKey, NodeValue>
where
    NodeKey: Hash + Eq,
{
    type Output = NodeValue;

    fn index(&self, index: NodeKey) -> &Self::Output {
        self.get(&index).expect("Node not found")
    }
}

impl<NodeKey, NodeValue> IndexMut<NodeKey> for AdjacencyVecGraph<NodeKey, NodeValue>
where
    NodeKey: Hash + Eq,
{
    fn index_mut(&mut self, index: NodeKey) -> &mut Self::Output {
        self.get_mut(&index).expect("Node not found")
    }
}
