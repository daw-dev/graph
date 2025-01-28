use crate::{
    topsort::TopSort,
    traversal::{PostOrderDFS, PreOrderDFS, BFS},
};
use std::hash::Hash;

pub trait ReferenceGraph {
    type NodeKey;
    fn adjacents(&self, node: &Self::NodeKey) -> impl Iterator<Item = &Self::NodeKey>;
    fn keys(&self) -> impl Iterator<Item = &Self::NodeKey>;

    fn pre_order_dfs<'a>(&'a self, root: &'a Self::NodeKey) -> PreOrderDFS<'a, Self::NodeKey, Self>
    where
        Self: Sized,
        Self::NodeKey: Hash + Eq,
    {
        PreOrderDFS::new(self, root)
    }
    fn post_order_dfs<'a>(&'a self, root: &'a Self::NodeKey) -> PostOrderDFS<'a, Self::NodeKey, Self>
    where
        Self: Sized,
        Self::NodeKey: Hash + Eq,
    {
        PostOrderDFS::new(self, root)
    }
    fn bfs<'a>(&'a self, root: &'a Self::NodeKey) -> BFS<'a, Self::NodeKey, Self>
    where
        Self: Sized,
        Self::NodeKey: Hash + Eq,
    {
        BFS::new(self, root)
    }
    fn top_sort(&self) -> TopSort<'_, Self::NodeKey>
    where
        Self: Sized,
        Self::NodeKey: Hash + Eq,
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
