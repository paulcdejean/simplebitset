use crate::bitset::BitSet;

impl BitSet {
    /// Creates a new bitset that contains every u8 value.
    pub fn total_set() -> Self {
        Self([u64::MAX; 4])
    }
}