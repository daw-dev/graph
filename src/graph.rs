use std::hash::Hash;
use crate::traversal::{PostOrderDFS, PreOrderDFS, BFS};

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