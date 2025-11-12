use crate::bitset::BITSET_ARRAY_SIZE;
use crate::bitset::BitSet;

impl BitSet {
    /// Clears the set, removing all values.
    pub fn clear(&mut self) {
        self.0 = [0; BITSET_ARRAY_SIZE]
    }
}
