use crate::bitset::BitSet;

impl BitSet {
    /// Returns true if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        let mut result: usize = 0;
        for bits in self.0 {
            result &= bits;
        }
        result == 0
    }
}
