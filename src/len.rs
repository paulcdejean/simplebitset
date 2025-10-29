use crate::bitset::BitSet;

impl BitSet {
    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        let mut result: usize = 0;
        for bits in self.0 {
            result += bits.count_ones() as usize;
        }
        result
    }
}
