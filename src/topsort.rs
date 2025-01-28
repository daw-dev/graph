use crate::{traversal::PostOrderDFS, ReferenceGraph};
use std::{collections::HashSet, hash::Hash};

pub struct TopSort<'a, NodeKey>
where
    NodeKey: Hash + Eq,
{
    stack: Vec<&'a NodeKey>,
}

impl<'a, NodeKey> TopSort<'a, NodeKey>
where
    NodeKey: Hash + Eq,
{
    pub fn new(graph: &'a impl ReferenceGraph<NodeKey = NodeKey>) -> Self {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();

        while let Some(next) = graph.keys().find(|id| !visited.contains(id)) {
            // TODO: remove clone and someway take the visited set by mutable reference
            for id in PostOrderDFS::with_visited(graph, next, visited.clone()) {
                visited.insert(id);
                stack.push(id);
            }
        }

        Self { stack }
    }
}

impl<'a, NodeKey> Iterator for TopSort<'a, NodeKey>
where
    NodeKey: Hash + Eq,
{
    type Item = &'a NodeKey;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}
