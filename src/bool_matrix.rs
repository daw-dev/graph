use crate::Graph;

impl<'a, const SIZE: usize> Graph<'a> for [[bool; SIZE]; SIZE] {
    type NodeId = usize;

    fn adjacents<'b: 'a>(&'b self, node: usize) -> impl Iterator<Item = usize> {
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

    fn iter<'b: 'a>(&'b self) -> impl Iterator<Item = usize> {
        0..SIZE
    }
}
