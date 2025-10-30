use crate::bitset::BitSet;

impl BitSet {
    /// Creates a new empty bitset.
    pub fn new() -> Self {
        Self([0; 4])
    }
    /// Creates a new bitset that contains every u8 value.
    pub fn total_set() -> Self {
        Self([u64::MAX; 4])
    }
}

impl Default for BitSet {
    /// An empty bitset.
    fn default() -> Self {
        Self([0; 4])
    }
}
