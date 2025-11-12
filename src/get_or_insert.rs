use crate::bitset::BitSet;

impl BitSet {
    /// Inserts the given value into the set if it is not present, then returns that value.
    pub fn get_or_insert(&mut self, value: u8) -> u8 {
        self.insert(value);
        value
    }
}
