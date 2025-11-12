use crate::bitset::BitSet;

impl BitSet {
    /// Creates a new empty bitset. The parameter can be any type.
    #[allow(unused_variables)]
    pub fn with_hasher<S>(hasher: S) -> Self {
        Self::new()
    }
}
