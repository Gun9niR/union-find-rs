use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::node::Node;
use crate::union_find::{Error, Result, UnionFind};

/// Disjoint sets data structure that implements union-find with
/// path compression and union by rank.
#[derive(Clone, Debug, Default)]
pub struct DisjointSets<T>
where
    T: Copy + Eq + Hash + Debug,
{
    nodes: HashMap<T, Node<T>>,
}

impl<T> DisjointSets<T>
where
    T: Copy + Eq + Hash + Debug,
{
    pub fn new() -> Self {
        DisjointSets {
            nodes: HashMap::new(),
        }
    }

    pub fn contains(&self, item: T) -> bool {
        self.nodes.contains_key(&item)
    }
}

impl<T> UnionFind<T> for DisjointSets<T>
where
    T: Copy + Eq + Hash + Debug,
{
    fn find_set(&mut self, item: T) -> Option<T> {
        let node = self.nodes.get(&item)?;
        Some(self.find_set_inner(node))
    }

    fn make_set(&mut self, item: T) -> Result<()> {
        if self.contains(item) {
            return Err(Error::ItemExists);
        }

        self.nodes.insert(item, Node::new(item));
        Ok(())
    }

    fn union(&mut self, x: T, y: T) -> Result<()> {
        let x_repr = self.find_set(x).ok_or(Error::ItemNotFound)?;
        let y_repr = self.find_set(y).ok_or(Error::ItemNotFound)?;

        if x_repr == y_repr {
            return Ok(());
        }

        let x_node = self.nodes.get(&x_repr).unwrap();
        let y_node = self.nodes.get(&y_repr).unwrap();
        let rank_sum = x_node.rank() + y_node.rank();

        if x_node.rank() < y_node.rank() {
            x_node.set_parent(y_repr);
            y_node.set_rank(rank_sum);
        } else {
            y_node.set_parent(x_repr);
            x_node.set_rank(rank_sum);
        }

        Ok(())
    }
}

impl<T> DisjointSets<T>
where
    T: Copy + Eq + Hash + Debug,
{
    /// Find the representative of the set containing `item`, performing path
    /// compression along the way.
    fn find_set_inner(&self, node: &Node<T>) -> T {
        if node.is_representative() {
            node.item()
        } else {
            let parent = self.nodes.get(&node.parent()).unwrap();
            let representative = self.find_set_inner(parent);
            node.set_parent(representative);
            representative
        }
    }
}

unsafe impl<T> Send for DisjointSets<T> where T: Copy + Eq + Hash + Debug {}
unsafe impl<T> Sync for DisjointSets<T> where T: Copy + Eq + Hash + Debug {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut sets = DisjointSets::new();

        // Find non-existent item.
        assert_eq!(sets.find_set(1), None);
        assert_eq!(sets.contains(1), false);

        sets.make_set(1).unwrap();
        sets.make_set(2).unwrap();
        sets.make_set(3).unwrap();
        sets.make_set(4).unwrap();
        sets.make_set(5).unwrap();

        assert_eq!(sets.contains(1), true);
        assert_eq!(sets.contains(2), true);
        assert_eq!(sets.contains(3), true);
        assert_eq!(sets.contains(4), true);
        assert_eq!(sets.contains(5), true);

        assert_eq!(sets.find_set(1), Some(1));
        assert_eq!(sets.find_set(2), Some(2));
        assert_eq!(sets.find_set(3), Some(3));
        assert_eq!(sets.find_set(4), Some(4));
        assert_eq!(sets.find_set(5), Some(5));

        // (1, 2), (3), (4), (5)
        sets.union(1, 2).unwrap();
        assert_eq!(sets.find_set(1), Some(1));
        assert_eq!(sets.find_set(2), Some(1));
        // (1, 2), (3, 4), (5)
        sets.union(3, 4).unwrap();
        assert_eq!(sets.find_set(3), Some(3));
        assert_eq!(sets.find_set(4), Some(3));
        // (1, 2, 3, 4), (5)
        sets.union(1, 3).unwrap();
        assert_eq!(sets.find_set(1), Some(1));
        assert_eq!(sets.find_set(2), Some(1));
        assert_eq!(sets.find_set(3), Some(1));
        assert_eq!(sets.find_set(4), Some(1));
        // (1, 2, 3, 4. 5)
        sets.union(1, 5).unwrap();
        assert_eq!(sets.find_set(1), Some(1));
        assert_eq!(sets.find_set(2), Some(1));
        assert_eq!(sets.find_set(3), Some(1));
        assert_eq!(sets.find_set(4), Some(1));
        assert_eq!(sets.find_set(5), Some(1));
    }
}
