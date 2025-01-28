use crate::ReferenceGraph;
use std::{collections::HashSet, hash::Hash};

pub struct PreOrderDFS<'a, NodeKey, G>
where
    NodeKey: Hash + Eq,
    G: ReferenceGraph<NodeKey = NodeKey>,
{
    graph: &'a G,
    visited: HashSet<&'a NodeKey>,
    stack: Vec<&'a NodeKey>,
}

impl<'a, NodeKey, G> PreOrderDFS<'a, NodeKey, G>
where
    NodeKey: Hash + Eq,
    G: ReferenceGraph<NodeKey = NodeKey>,
{
    pub fn new(graph: &'a G, root: &'a NodeKey) -> Self {
        Self {
            graph,
            visited: HashSet::new(),
            stack: vec![root],
        }
    }

    pub fn with_visited(graph: &'a G, root: &'a NodeKey, visited: HashSet<&'a NodeKey>) -> Self {
        Self {
            graph,
            visited,
            stack: vec![root],
        }
    }
}

impl<'a, NodeKey, G> Iterator for PreOrderDFS<'a, NodeKey, G>
where
    NodeKey: Hash + Eq,
    G: ReferenceGraph<NodeKey = NodeKey>,
{
    type Item = &'a NodeKey;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.stack.pop()?;

        self.visited.insert(current);

        for adj in self.graph.adjacents(current) {
            if !self.visited.contains(&adj) {
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

pub struct PostOrderDFS<'a, NodeKey, G>
where
    NodeKey: Hash + Eq,
    G: ReferenceGraph<NodeKey = NodeKey>,
{
    graph: &'a G,
    visited: HashSet<&'a NodeKey>,
    stack: Vec<(&'a NodeKey, VisitTag)>,
}

impl<'a, NodeKey, G> PostOrderDFS<'a, NodeKey, G>
where
    NodeKey: Hash + Eq,
    G: ReferenceGraph<NodeKey = NodeKey>,
{
    pub fn new(graph: &'a G, root: &'a NodeKey) -> Self {
        Self {
            graph,
            visited: HashSet::new(),
            stack: vec![(root, VisitTag::Discovered)],
        }
    }

    pub fn with_visited(graph: &'a G, root: &'a NodeKey, visited: HashSet<&'a NodeKey>) -> Self {
        Self {
            graph,
            visited,
            stack: vec![(root, VisitTag::Discovered)],
        }
    }
}

impl<'a, NodeKey, G> Iterator for PostOrderDFS<'a, NodeKey, G>
where
    NodeKey: Hash + Eq,
    G: ReferenceGraph<NodeKey = NodeKey>,
{
    type Item = &'a NodeKey;

    fn next(&mut self) -> Option<Self::Item> {
        let (current, tag) = self.stack.pop()?;

        match tag {
            VisitTag::Discovered => {
                self.stack.push((current, VisitTag::Finished));

                self.visited.insert(current);

                for adj in self.graph.adjacents(current) {
                    if !self.visited.contains(&adj) {
                        self.stack.push((adj, VisitTag::Discovered));
                    }
                }

                self.next()
            }
            VisitTag::Finished => Some(current),
        }
    }
}
