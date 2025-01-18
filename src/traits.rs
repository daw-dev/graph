use std::{collections::{HashSet, VecDeque}, hash::Hash};

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
    fn new(graph: &'a G, root: &'a NodeId) -> Self {
        Self {
            graph,
            visited: HashSet::new(),
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
    fn new(graph: &'a G, root: &'a NodeId) -> Self {
        Self {
            graph,
            visited: HashSet::new(),
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

pub struct BFS<'a, NodeId, G>
where
    NodeId: Hash + Eq,
    G: Graph<NodeId = NodeId>,
{
    graph: &'a G,
    visited: HashSet<&'a NodeId>,
    queue: VecDeque<&'a NodeId>,
}

impl<'a, NodeId, G> BFS<'a, NodeId, G>
where
    NodeId: Hash + Eq,
    G: Graph<NodeId = NodeId>,
{
    fn new(graph: &'a G, root: &'a NodeId) -> Self {
        Self {
            graph,
            visited: HashSet::new(),
            queue: [root].into(),
        }
    }
}

impl<'a, NodeId, G> Iterator for BFS<'a, NodeId, G>
where
    NodeId: Hash + Eq,
    G: Graph<NodeId = NodeId>,
{
    type Item = &'a NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.queue.pop_front()?;

        self.visited.insert(current);

        for adj in self.graph.adjacents(current) {
            if !self.visited.contains(adj) {
                self.queue.push_back(adj);
            }
        }

        Some(current)
    }
}

pub trait Graph {
    type NodeId;

    fn adjacents(&self, node: &Self::NodeId) -> impl Iterator<Item = &Self::NodeId>;
    fn iter(&self) -> impl Iterator<Item = &Self::NodeId>;
    fn pre_order_dfs<'a>(
        &'a self,
        root: &'a Self::NodeId,
    ) -> PreOrderDFS<'a, Self::NodeId, Self>
    where
        Self: Sized,
        Self::NodeId: Hash + Eq,
    {
        PreOrderDFS::new(self, root)
    }
    fn post_order_dfs<'a>(
        &'a self,
        root: &'a Self::NodeId,
    ) -> PostOrderDFS<'a, Self::NodeId, Self>
    where
        Self: Sized,
        Self::NodeId: Hash + Eq,
    {
        PostOrderDFS::new(self, root)
    }
    fn bfs<'a>(
        &'a self,
        root: &'a Self::NodeId,
    ) -> BFS<'a, Self::NodeId, Self>
    where
        Self: Sized,
        Self::NodeId: Hash + Eq,
    {
        BFS::new(self, root)
    }
}