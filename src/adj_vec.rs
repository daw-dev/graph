use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use super::traits::Graph;

pub struct AdjacencyVecGraph<Id, Node>
where
    Id: Hash + Eq,
{
    matrix: HashMap<Id, (Node, HashSet<Id>)>,
}

impl<Id, Node> AdjacencyVecGraph<Id, Node>
where
    Id: Hash + Eq,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_node(&mut self, node_id: Id, node_info: Node) {
        self.matrix.insert(node_id, (node_info, HashSet::new()));
    }

    pub fn connect(&mut self, from: Id, to: Id) {
        if from == to {
            return;
        }

        if let Some((_, adjacents)) = self.matrix.get_mut(&from) {
            adjacents.insert(to);
        }
    }
}

impl<Id, Node> Graph for AdjacencyVecGraph<Id, Node>
where
    Id: Hash + Eq,
{
    type NodeIdType = Id;

    type NodeType = Node;

    fn adjacents(&self, node: &Self::NodeIdType) -> impl Iterator<Item = &Self::NodeIdType> {
        self.matrix[node].1.iter()
    }

    fn iter(&self) -> impl Iterator<Item = &Self::NodeIdType> {
        self.matrix.keys()
    }

    fn node_info(&self, node: &Self::NodeIdType) -> &Self::NodeType {
        &self.matrix[node].0
    }
}

impl<Id, Node> Debug for AdjacencyVecGraph<Id, Node>
where
    Id: Hash + Eq + Debug,
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

impl<Id, Node> Default for AdjacencyVecGraph<Id, Node>
where
    Id: Hash + Eq,
{
    fn default() -> Self {
        Self {
            matrix: Default::default(),
        }
    }
}
