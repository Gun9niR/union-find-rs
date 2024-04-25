use std::cell::Cell;
use std::fmt::Debug;
use std::hash::Hash;

/// Node is a wrapper around a element in the disjoin sets with parent and rank.
#[derive(Clone, Debug)]
pub struct Node<T: Copy> {
    item: T,
    // Use `Cell` for internal mutability.
    /// A node is the representative of the set if its parent is itself.
    parent: Cell<T>,
    rank: Cell<usize>,
}

impl<T> Node<T>
where
    T: Copy + Eq + Hash + Debug,
{
    pub fn new(item: T) -> Self {
        Node {
            item,
            parent: item.into(),
            rank: 1.into(),
        }
    }

    pub fn item(&self) -> T {
        self.item
    }

    pub fn parent(&self) -> T {
        self.parent.get()
    }

    pub fn set_parent(&self, parent: T) {
        self.parent.set(parent);
    }

    pub fn rank(&self) -> usize {
        self.rank.get()
    }

    pub fn set_rank(&self, rank: usize) {
        self.rank.set(rank);
    }

    pub fn is_representative(&self) -> bool {
        self.item == self.parent.get()
    }
}

impl<T> AsRef<T> for Node<T>
where
    T: Copy + Eq + Hash + Debug,
{
    fn as_ref(&self) -> &T {
        &self.item
    }
}
