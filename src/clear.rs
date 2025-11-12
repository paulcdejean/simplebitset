use crate::bitset::BitSet;
use crate::bitset::BITSET_ARRAY_SIZE;

impl BitSet {
    /// Clears the set, removing all values.
    pub fn clear(&mut self) {
        self.0 = [0; BITSET_ARRAY_SIZE]
    }
}
