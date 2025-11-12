use crate::bitset::BITSET_ARRAY_SIZE;
use crate::bitset::BitSet;

impl BitSet {
    /// Creates a new empty bitset.
    pub fn new() -> Self {
        Self([0; BITSET_ARRAY_SIZE])
    }
}

impl Default for BitSet {
    /// An empty bitset.
    fn default() -> Self {
        Self::new()
    }
}
