use std::hash::Hash;
use crate::{topsort::TopSort, traversal::{PostOrderDFS, PreOrderDFS, BFS}};

pub trait Graph<'a> {
    type NodeId: Copy + 'a;

    fn adjacents(&'a self, node: Self::NodeId) -> impl Iterator<Item = Self::NodeId>;
    fn iter(&'a self) -> impl Iterator<Item = Self::NodeId>;
    fn pre_order_dfs(
        &'a self,
        root: Self::NodeId,
    ) -> PreOrderDFS<'a, Self::NodeId, Self>
    where
        Self: Sized,
        Self::NodeId: Hash + Eq,
    {
        PreOrderDFS::new(self, root)
    }
    fn post_order_dfs(
        &'a self,
        root: Self::NodeId,
    ) -> PostOrderDFS<'a, Self::NodeId, Self>
    where
        Self: Sized,
        Self::NodeId: Hash + Eq,
    {
        PostOrderDFS::new(self, root)
    }
    fn bfs(
        &'a self,
        root: Self::NodeId,
    ) -> BFS<'a, Self::NodeId, Self>
    where
        Self: Sized,
        Self::NodeId: Hash + Eq,
    {
        BFS::new(self, root)
    }
    fn top_sort(&'a self) -> TopSort<'a, Self::NodeId>
    where
        Self: Sized,
        Self::NodeId: Hash + Eq,
    {
        TopSort::new(self)
    }
}

impl<'a, const SIZE: usize> Graph<'a> for [[bool; SIZE]; SIZE] {
    type NodeId = usize;

    fn adjacents(&'a self, node: usize) -> impl Iterator<Item = usize> {
        self[node].iter().enumerate().filter_map(|(idx, &is_adj)| {
            if is_adj {
                Some(idx)
            } else {
                None
            }
        })
    }

    fn iter(&'a self) -> impl Iterator<Item = usize> {
        0..SIZE
    }
}