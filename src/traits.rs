use std::{collections::{HashSet, VecDeque}, hash::Hash};

pub struct PreOrderDFS<'a, Id, G>
where
    Id: Hash + Eq,
    G: Graph<NodeIdType = Id>,
{
    graph: &'a G,
    visited: HashSet<&'a Id>,
    stack: Vec<&'a Id>,
}

impl<'a, Id, G> PreOrderDFS<'a, Id, G>
where
    Id: Hash + Eq,
    G: Graph<NodeIdType = Id>,
{
    fn new(graph: &'a G, root: &'a Id) -> Self {
        Self {
            graph,
            visited: HashSet::new(),
            stack: vec![root],
        }
    }
}

impl<'a, Id, G> Iterator for PreOrderDFS<'a, Id, G>
where
    Id: Hash + Eq,
    G: Graph<NodeIdType = Id>,
{
    type Item = &'a Id;

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

pub struct PostOrderDFS<'a, Id, G>
where
    Id: Hash + Eq,
    G: Graph<NodeIdType = Id>,
{
    graph: &'a G,
    visited: HashSet<&'a Id>,
    stack: Vec<(&'a Id, VisitTag)>,
}

impl<'a, Id, G> PostOrderDFS<'a, Id, G>
where
    Id: Hash + Eq,
    G: Graph<NodeIdType = Id>,
{
    fn new(graph: &'a G, root: &'a Id) -> Self {
        Self {
            graph,
            visited: HashSet::new(),
            stack: vec![(root, VisitTag::Discovered)],
        }
    }
}

impl<'a, Id, G> Iterator for PostOrderDFS<'a, Id, G>
where
    Id: Hash + Eq,
    G: Graph<NodeIdType = Id>,
{
    type Item = &'a Id;

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

pub struct BFS<'a, Id, G>
where
    Id: Hash + Eq,
    G: Graph<NodeIdType = Id>,
{
    graph: &'a G,
    visited: HashSet<&'a Id>,
    queue: VecDeque<&'a Id>,
}

impl<'a, Id, G> BFS<'a, Id, G>
where
    Id: Hash + Eq,
    G: Graph<NodeIdType = Id>,
{
    fn new(graph: &'a G, root: &'a Id) -> Self {
        Self {
            graph,
            visited: HashSet::new(),
            queue: [root].into(),
        }
    }
}

impl<'a, Id, G> Iterator for BFS<'a, Id, G>
where
    Id: Hash + Eq,
    G: Graph<NodeIdType = Id>,
{
    type Item = &'a Id;

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
    type NodeIdType;
    type NodeType;

    fn adjacents(&self, node: &Self::NodeIdType) -> impl Iterator<Item = &Self::NodeIdType>;
    fn iter(&self) -> impl Iterator<Item = &Self::NodeIdType>;
    fn node_info(&self, node: &Self::NodeIdType) -> &Self::NodeType;
    fn pre_order_dfs<'a>(
        &'a self,
        root: &'a Self::NodeIdType,
    ) -> PreOrderDFS<'a, Self::NodeIdType, Self>
    where
        Self: Sized,
        Self::NodeIdType: Hash + Eq,
    {
        PreOrderDFS::new(self, root)
    }
    fn post_order_dfs<'a>(
        &'a self,
        root: &'a Self::NodeIdType,
    ) -> PostOrderDFS<'a, Self::NodeIdType, Self>
    where
        Self: Sized,
        Self::NodeIdType: Hash + Eq,
    {
        PostOrderDFS::new(self, root)
    }
    fn bfs<'a>(
        &'a self,
        root: &'a Self::NodeIdType,
    ) -> BFS<'a, Self::NodeIdType, Self>
    where
        Self: Sized,
        Self::NodeIdType: Hash + Eq,
    {
        BFS::new(self, root)
    }
}