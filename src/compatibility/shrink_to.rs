use crate::bitset::BitSet;

impl BitSet {
    /// Does nothing. The capacity of a simple BitSet is fixed at 256 bits.
    #[allow(unused_variables)]
    pub fn shrink_to(&mut self, min_capacity: usize) {}
}
