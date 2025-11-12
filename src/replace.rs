use crate::bitset::BitSet;

impl BitSet {
    /// Adds a value to the set, replacing the existing value, if any, that is equal to the given one. Returns the replaced value.
    pub fn replace(&mut self, value: u8) -> Option<u8> {
        if self.insert(value) {
            None
        } else {
            Some(value)
        }
    }
}
