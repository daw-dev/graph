use std::{collections::HashSet, hash::Hash};
use crate::Graph;

pub struct PreOrderDFS<'a, NodeId, G>
where
    NodeId: Hash + Eq,
    G: Graph<NodeId = NodeId>,
{
    graph: &'a G,
    visited: HashSet<&'a NodeId>,
    stack: Vec<&'a NodeId>,
}

impl<'a, NodeId, G> PreOrderDFS<'a, NodeId, G>
where
    NodeId: Hash + Eq,
    G: Graph<NodeId = NodeId>,
{
    pub fn new(graph: &'a G, root: &'a NodeId) -> Self {
        Self {
            graph,
            visited: HashSet::new(),
            stack: vec![root],
        }
    }

    pub fn with_visited(graph: &'a G, root: &'a NodeId, visited: HashSet<&'a NodeId>) -> Self {
        Self {
            graph,
            visited,
            stack: vec![root],
        }
    }
}

impl<'a, NodeId, G> Iterator for PreOrderDFS<'a, NodeId, G>
where
    NodeId: Hash + Eq,
    G: Graph<NodeId = NodeId>,
{
    type Item = &'a NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.stack.pop()?;

        self.visited.insert(current);

        for adj in self.graph.adjacents(current) {
            if !self.visited.contains(adj) {
                self.stack.push(adj);
            }
        }

        Some(current)
    }
}

enum VisitTag {
    Discovered,
    Finished,
}

pub struct PostOrderDFS<'a, NodeId, G>
where
    NodeId: Hash + Eq,
    G: Graph<NodeId = NodeId>,
{
    graph: &'a G,
    visited: HashSet<&'a NodeId>,
    stack: Vec<(&'a NodeId, VisitTag)>,
}

impl<'a, NodeId, G> PostOrderDFS<'a, NodeId, G>
where
    NodeId: Hash + Eq,
    G: Graph<NodeId = NodeId>,
{
    pub fn new(graph: &'a G, root: &'a NodeId) -> Self {
        Self {
            graph,
            visited: HashSet::new(),
            stack: vec![(root, VisitTag::Discovered)],
        }
    }

    pub fn with_visited(graph: &'a G, root: &'a NodeId, visited: HashSet<&'a NodeId>) -> Self {
        Self {
            graph,
            visited,
            stack: vec![(root, VisitTag::Discovered)],
        }
    }
}

impl<'a, NodeId, G> Iterator for PostOrderDFS<'a, NodeId, G>
where
    NodeId: Hash + Eq,
    G: Graph<NodeId = NodeId>,
{
    type Item = &'a NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let (current, tag) = self.stack.pop()?;

        match tag {
            VisitTag::Discovered => {
                self.stack.push((current, VisitTag::Finished));

                self.visited.insert(current);

                for adj in self.graph.adjacents(current) {
                    if !self.visited.contains(adj) {
                        self.stack.push((adj, VisitTag::Discovered));
                    }
                }

                self.next()
            }
            VisitTag::Finished => Some(current),
        }
    }
}

