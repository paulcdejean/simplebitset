use crate::bitset::BitSet;

impl BitSet {
    /// Returns 256, which is the number of bits our simple bitset can store.
    pub fn capacity(&self) -> usize {
        256
    }
}
