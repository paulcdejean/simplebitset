use crate::bitset::BitSet;

impl BitSet {
    /// Removes and returns the value in the set, if any, that is equal to the given one.
    pub fn take(&mut self, value: u8) -> Option<u8> {
        if self.remove(value) {
            Some(value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take() {
        let mut example: BitSet = BitSet::new();
        assert!(!example.contains(7));
        example.insert(7);
        assert!(example.contains(7));
        assert_eq!(Some(7), example.take(7));
        assert!(!example.contains(7));
        assert!(!example.contains(6));
    }
}
