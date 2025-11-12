use crate::bitset::BitSet;

impl BitSet {
    /// Creates a new empty bitset. The hasher parameter can be any type.
    #[allow(unused_variables)]
    pub fn with_capacity_and_hasher<S>(capacity: usize, hasher: S) -> Self {
        Self::new()
    }
}
