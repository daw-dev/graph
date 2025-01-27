use crate::{
    topsort::TopSort,
    traversal::{PostOrderDFS, PreOrderDFS, BFS},
};
use std::hash::Hash;

pub trait ReferenceGraph {
    type NodeId;
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
    fn top_sort(&self) -> TopSort<'_, Self::NodeId>
    where
        Self: Sized,
        Self::NodeId: Hash + Eq,
    {
        TopSort::new(self)
    }
}

pub trait CopyGraph {
    type NodeId;

    fn adjacents(&self, node: Self::NodeId) -> impl Iterator<Item = Self::NodeId>;
    fn iter(&self) -> impl Iterator<Item = Self::NodeId>;

    // fn pre_order_dfs(&self, root: Self::NodeId) -> PreOrderDFS<'_, Self::NodeId, Self>
    // where
    //     Self: Sized,
    //     Self::NodeId: Hash + Eq,
    // {
    //     todo!()
    // }
    // fn post_order_dfs(&self, root: Self::NodeId) -> PostOrderDFS<'_, Self::NodeId, Self>
    // where
    //     Self: Sized,
    //     Self::NodeId: Hash + Eq,
    // {
    //     todo!()
    // }
    // fn bfs(&self, root: Self::NodeId) -> BFS<'_, Self::NodeId, Self>
    // where
    //     Self: Sized,
    //     Self::NodeId: Hash + Eq,
    // {
    //     todo!()
    // }
    fn top_sort(&self) -> TopSort<'_, Self::NodeId>
    where
        Self: Sized,
        Self::NodeId: Hash + Eq,
    {
        todo!()
    }
}
