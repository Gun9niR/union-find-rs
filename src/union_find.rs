pub trait UnionFind<T: Copy> {
    /// Find the representative of the set containing `item`, if any.
    ///
    /// If item does not exist, return `None`.
    fn find_set(&mut self, item: T) -> Option<T>;

    /// Create a new set containing only `item`. If `item` already exists in
    /// the disjoint sets, an error is returned.
    fn make_set(&mut self, item: T) -> Result<()>;

    /// Merge the sets containing `x` and `y`. If `x` or `y` do not exist in
    /// the disjoint sets, an error is returned.
    fn union(&mut self, x: T, y: T) -> Result<()>;
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// The item is not in the disjoint sets.
    ItemNotFound,
    /// The item is already in the disjoint sets.
    ItemExists,
}
