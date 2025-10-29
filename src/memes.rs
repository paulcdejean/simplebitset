use crate::bitset::BitSet;

impl BitSet {
    /// Returns 256, which is the number of bits our simple bitset can store.
    pub fn capacity(&self) -> usize {
        256
    }
    /// Clears the set, removing all values.
    pub fn clear(&mut self) {
        self.0 = [0; 4]
    }
    /// Inserts the given value into the set if it is not present, then returns that value.
    pub fn get_or_insert(&mut self, value: u8) -> u8 {
        self.insert(value);
        value
    }
}
