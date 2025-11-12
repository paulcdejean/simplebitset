use crate::bitset::BitSet;

impl BitSet {
    /// Adds a value to the set.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// - If the set did not previously contain this value, `true` is returned.
    /// - If the set already contained this value, `false` is returned.
    #[inline] // Required because MIR doesn't know u8::MAX / 4 is between 0 and 3.
    pub fn insert(&mut self, value: u8) -> bool {
        let index: u8 = value / 64;
        let offset: u8 = value % 64;
        let num: &mut u64 = self.0.get_mut(index as usize).unwrap();
        let mask: u64 = 1 << offset;
        if *num & mask == 0 {
            *num |= mask;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut example: BitSet = BitSet::new();
        assert!(example.insert(0));
        assert!(!example.insert(0));
        assert!(example.contains(0));
        assert!(!example.contains(1));
        assert!(!example.contains(255));
        assert!(example.insert(69));
        assert!(!example.insert(69));
        assert!(example.contains(69));
        assert!(example.insert(255));
        assert!(!example.insert(255));
        assert!(example.contains(255));
    }
}
