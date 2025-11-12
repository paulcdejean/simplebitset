use crate::bitset::BitSet;

impl BitSet {
    /// Does nothing. The capacity of a simple BitSet is fixed at 256 bits.
    #[allow(unused_variables)]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), core::convert::Infallible> {
        Ok(())
    }
}
