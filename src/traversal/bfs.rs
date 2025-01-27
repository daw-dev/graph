use crate::Graph;
use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

pub struct BFS<'a, NodeId, G>
where
    NodeId: Hash + Eq + Copy,
    G: Graph<NodeId = NodeId>,
{
    graph: &'a G,
    visited: HashSet<NodeId>,
    queue: VecDeque<NodeId>,
}

impl<'a, NodeId, G> BFS<'a, NodeId, G>
where
    NodeId: Hash + Eq + Copy,
    G: Graph<NodeId = NodeId>,
{
    pub fn new(graph: &'a G, root: NodeId) -> Self {
        Self {
            graph,
            visited: HashSet::new(),
            queue: [root].into(),
        }
    }

    pub fn with_visited(graph: &'a G, root: NodeId, visited: HashSet<NodeId>) -> Self {
        Self {
            graph,
            visited,
            queue: [root].into(),
        }
    }
}

impl<'a, NodeId, G> Iterator for BFS<'a, NodeId, G>
where
    NodeId: Hash + Eq + Copy,
    G: Graph<NodeId = NodeId>,
{
    type Item = NodeId;

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
