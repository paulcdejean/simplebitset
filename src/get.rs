use crate::bitset::BitSet;

impl BitSet {
    /// Returns the value in the set, if any, that is equal to the given value.
    pub fn get(&self, value: u8) -> Option<u8> {
        if self.contains(value) {
            Some(value)
        } else {
            None
        }
    }
}
