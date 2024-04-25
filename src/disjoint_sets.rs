use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use crate::node::Node;
use crate::union_find::{Error, Result, UnionFind};

// Store IDs in the disjoint sets instead of the items to workaround mutability
// issues with `Cell`.
type Id = u64;

/// Disjoint sets data structure that implements union-find with
/// path compression and union by rank.
#[derive(Clone, Debug, Default)]
pub struct DisjointSets<T> {
    nodes: HashMap<Id, Node<Id>>,
    item_to_id: HashMap<T, Id>,
    next_id: Id,
}

impl<T> DisjointSets<T>
where
    T: Eq + Hash,
{
    pub fn new() -> Self {
        DisjointSets {
            nodes: HashMap::new(),
            item_to_id: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn contains(&self, item: &T) -> bool {
        self.item_to_id.get(item).is_some()
    }
}

impl<T> UnionFind<T> for DisjointSets<T>
where
    T: Eq + Hash,
{
    fn same_set(&mut self, x: &T, y: &T) -> Result<bool> {
        let x_id = *self.item_to_id.get(x).ok_or(Error::ItemNotFound)?;
        let y_id = *self.item_to_id.get(y).ok_or(Error::ItemNotFound)?;
        let x_repr = self.find_repr(x_id);
        let y_repr = self.find_repr(y_id);
        Ok(x_repr == y_repr)
    }

    fn make_set(&mut self, item: T) -> Result<()> {
        if self.contains(&item) {
            return Err(Error::ItemExists);
        }

        let id = self.next_id;
        self.next_id += 1;
        self.item_to_id.insert(item, id);

        self.nodes.insert(id, Node::new(id));
        Ok(())
    }

    fn union(&mut self, x: &T, y: &T) -> Result<()> {
        let x_id = *self.item_to_id.get(x).ok_or(Error::ItemNotFound)?;
        let y_id = *self.item_to_id.get(y).ok_or(Error::ItemNotFound)?;
        let x_repr = self.find_repr(x_id);
        let y_repr = self.find_repr(y_id);

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

impl<T> DisjointSets<T> {
    /// Find the representative of the set containing `id`, performing path
    /// compression along the way.
    ///
    /// Assumes `id` exists.
    fn find_repr(&mut self, id: Id) -> Id {
        let node = self.nodes.get(&id).unwrap();
        self.find_repr_inner(node)
    }

    fn find_repr_inner(&self, node: &Node<Id>) -> Id {
        if node.is_representative() {
            node.item()
        } else {
            let parent = self.nodes.get(&node.parent()).unwrap();
            let representative = self.find_repr_inner(parent);
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
        assert_eq!(sets.contains(&1), false);

        sets.make_set(1).unwrap();
        sets.make_set(2).unwrap();
        sets.make_set(3).unwrap();
        sets.make_set(4).unwrap();
        sets.make_set(5).unwrap();

        assert!(sets.contains(&1));
        assert!(sets.contains(&2));
        assert!(sets.contains(&3));
        assert!(sets.contains(&4));
        assert!(sets.contains(&5));

        assert!(!sets.same_set(&1, &2).unwrap());
        assert!(!sets.same_set(&1, &3).unwrap());
        assert!(!sets.same_set(&2, &3).unwrap());
        assert!(sets.same_set(&1, &1).unwrap());

        // (1, 2), (3), (4), (5)
        sets.union(&1, &2).unwrap();
        assert!(sets.same_set(&1, &2).unwrap());
        assert!(!sets.same_set(&1, &3).unwrap());
        // (1, 2), (3, 4), (5)
        sets.union(&3, &4).unwrap();
        assert!(sets.same_set(&3, &4).unwrap());
        assert!(!sets.same_set(&1, &3).unwrap());
        // (1, 2, 3, 4), (5)
        sets.union(&1, &3).unwrap();
        assert!(sets.same_set(&1, &2).unwrap());
        assert!(sets.same_set(&1, &4).unwrap());
        assert!(sets.same_set(&2, &3).unwrap());
        assert!(sets.same_set(&2, &4).unwrap());
        assert!(!sets.same_set(&4, &5).unwrap());
        // (1, 2, 3, 4. 5)
        sets.union(&1, &5).unwrap();
        assert!(sets.same_set(&1, &2).unwrap());
        assert!(sets.same_set(&2, &3).unwrap());
        assert!(sets.same_set(&3, &4).unwrap());
        assert!(sets.same_set(&4, &5).unwrap());
        assert!(sets.same_set(&1, &5).unwrap());
    }
}
