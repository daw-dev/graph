use crate::{traversal::PostOrderDFS, Graph};
use std::{collections::HashSet, hash::Hash, marker::PhantomData};

pub struct TopSort<'a, NodeId>
where
    NodeId: Hash + Eq,
{
    stack: Vec<NodeId>,
    phantom: PhantomData<&'a NodeId>,
}

impl<'a, NodeId> TopSort<'a, NodeId>
where
    NodeId: Hash + Eq + Copy,
{
    pub fn new(graph: &'a impl Graph<NodeId = NodeId>) -> Self {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();

        while let Some(next) = graph.iter().find(|id| !visited.contains(id)) {
            // TODO: remove clone and someway take the visited set by mutable reference
            for id in PostOrderDFS::with_visited(graph, next, visited.clone()) {
                visited.insert(id);
                stack.push(id);
            }
        }

        Self {
            stack,
            phantom: PhantomData,
        }
    }
}

impl<'a, NodeId> Iterator for TopSort<'a, NodeId>
where
    NodeId: Hash + Eq + Copy,
{
    type Item = NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}
