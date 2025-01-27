use crate::graph::{CopyGraph, Graph};

impl<const SIZE: usize> Graph for [[bool; SIZE]; SIZE] {
    type NodeId = usize;
}

impl<const SIZE: usize> CopyGraph for [[bool; SIZE]; SIZE] {
    fn adjacents(&self, node: usize) -> impl Iterator<Item = usize> {
        self[node].iter().enumerate().filter_map(
            |(idx, &is_adj)| {
                if is_adj {
                    Some(idx)
                } else {
                    None
                }
            },
        )
    }

    fn iter(&self) -> impl Iterator<Item = usize> {
        0..SIZE
    }
}
