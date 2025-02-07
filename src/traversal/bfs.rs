use crate::ReferenceGraph;
use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

pub struct BFS<'a, NodeKey, G>
where
    NodeKey: Hash + Eq,
    G: ReferenceGraph<NodeKey = NodeKey>,
{
    graph: &'a G,
    visited: HashSet<&'a NodeKey>,
    queue: VecDeque<&'a NodeKey>,
}

impl<'a, NodeKey, G> BFS<'a, NodeKey, G>
where
    NodeKey: Hash + Eq,
    G: ReferenceGraph<NodeKey = NodeKey>,
{
    pub fn new(graph: &'a G, root: &'a NodeKey) -> Self {
        Self {
            graph,
            visited: HashSet::new(),
            queue: [root].into(),
        }
    }

    pub fn with_visited(graph: &'a G, root: &'a NodeKey, visited: HashSet<&'a NodeKey>) -> Self {
        Self {
            graph,
            visited,
            queue: [root].into(),
        }
    }
}

impl<'a, NodeKey, G> Iterator for BFS<'a, NodeKey, G>
where
    NodeKey: Hash + Eq,
    G: ReferenceGraph<NodeKey = NodeKey>,
{
    type Item = &'a NodeKey;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.queue.pop_front()?;

        
        for adj in self.graph.adjacents(current) {
            if !self.visited.contains(&adj) {
                self.visited.insert(&adj);
                self.queue.push_back(adj);
            }
        }

        Some(current)
    }
}
