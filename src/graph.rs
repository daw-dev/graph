use crate::{
    topsort::TopSort,
    traversal::{PostOrderDFS, PreOrderDFS, BFS},
};
use std::hash::Hash;

pub trait Graph {
    type NodeId;
}

pub trait ReferenceGraph: Graph {
    fn adjacents(&self, node: &Self::NodeId) -> impl Iterator<Item = &Self::NodeId>;
    fn iter(&self) -> impl Iterator<Item = &Self::NodeId>;
    fn pre_order_dfs<'a>(&'a self, root: &'a Self::NodeId) -> PreOrderDFS<'a, Self::NodeId, Self>
    where
        Self: Sized,
        Self::NodeId: Hash + Eq,
    {
        PreOrderDFS::new(self, root)
    }
    fn post_order_dfs<'a>(&'a self, root: &'a Self::NodeId) -> PostOrderDFS<'a, Self::NodeId, Self>
    where
        Self: Sized,
        Self::NodeId: Hash + Eq,
    {
        PostOrderDFS::new(self, root)
    }
    fn bfs<'a>(&'a self, root: &'a Self::NodeId) -> BFS<'a, Self::NodeId, Self>
    where
        Self: Sized,
        Self::NodeId: Hash + Eq,
    {
        BFS::new(self, root)
    }
    fn top_sort<'a>(&'a self) -> TopSort<'a, Self::NodeId>
    where
        Self: Sized,
        Self::NodeId: Hash + Eq,
    {
        TopSort::new(self)
    }
}

pub trait CopyGraph: Graph {
    fn adjacents(&self, node: Self::NodeId) -> impl Iterator<Item = Self::NodeId>;
    fn iter(&self) -> impl Iterator<Item = Self::NodeId>;
    fn pre_order_dfs<'a>(&'a self, root: Self::NodeId) -> PreOrderDFS<'a, Self::NodeId, Self>
    where
        Self: Sized,
        Self::NodeId: Hash + Eq,
    {
        todo!()
    }
    fn post_order_dfs<'a>(&'a self, root: Self::NodeId) -> PostOrderDFS<'a, Self::NodeId, Self>
    where
        Self: Sized,
        Self::NodeId: Hash + Eq,
    {
        todo!()
    }
    fn bfs<'a>(&'a self, root: Self::NodeId) -> BFS<'a, Self::NodeId, Self>
    where
        Self: Sized,
        Self::NodeId: Hash + Eq,
    {
        todo!()
    }
    fn top_sort<'a>(&'a self) -> TopSort<'a, Self::NodeId>
    where
        Self: Sized,
        Self::NodeId: Hash + Eq,
    {
        todo!()
    }
}
