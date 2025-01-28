use crate::ReferenceGraph;
use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

pub struct BFS<'a, NodeId, G>
where
    NodeId: Hash + Eq,
    G: ReferenceGraph<NodeKey = NodeId>,
{
    graph: &'a G,
    visited: HashSet<&'a NodeId>,
    queue: VecDeque<&'a NodeId>,
}

impl<'a, NodeId, G> BFS<'a, NodeId, G>
where
    NodeId: Hash + Eq,
    G: ReferenceGraph<NodeKey = NodeId>,
{
    pub fn new(graph: &'a G, root: &'a NodeId) -> Self {
        Self {
            graph,
            visited: HashSet::new(),
            queue: [root].into(),
        }
    }

    pub fn with_visited(graph: &'a G, root: &'a NodeId, visited: HashSet<&'a NodeId>) -> Self {
        Self {
            graph,
            visited,
            queue: [root].into(),
        }
    }
}

impl<'a, NodeId, G> Iterator for BFS<'a, NodeId, G>
where
    NodeId: Hash + Eq,
    G: ReferenceGraph<NodeKey = NodeId>,
{
    type Item = &'a NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.queue.pop_front()?;

        self.visited.insert(current);

        for adj in self.graph.adjacents(current) {
            if !self.visited.contains(&adj) {
                self.queue.push_back(adj);
            }
        }

        Some(current)
    }
}
