use crate::bitset::BitSet;

impl BitSet {
    /// Adds a value to the set.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// - If the set did not previously contain this value, `true` is returned.
    /// - If the set already contained this value, `false` is returned.
    pub fn insert(&mut self, value: u8) -> bool {
        let index: u8 = value / 64;
        let offset: u8 = value % 64;
        // SAFETY: a u8 divided by 64 is between 0 and 3. MIR doesn't know this though.
        let num: &mut u64 = unsafe { self.0.get_unchecked_mut(index as usize) };
        let mask: u64 = 1 << offset;
        if *num & mask == 0 {
            *num |= mask;
            true
        } else {
            false
        }
    }

    /// Older version of insert with worse performance.
    pub fn insert_old(&mut self, value: u8) -> bool {
        match value {
            0..=63 => {
                let nth: u64 = 1 << value;
                if nth & self.0[0] == 0 {
                    self.0[0] |= nth;
                    true
                } else {
                    false
                }
            }
            64..=127 => {
                let nth: u64 = 1 << (value - 64);
                if nth & self.0[1] == 0 {
                    self.0[1] |= nth;
                    true
                } else {
                    false
                }
            }
            128..=191 => {
                let nth: u64 = 1 << (value - 128);
                if nth & self.0[2] == 0 {
                    self.0[2] |= nth;
                    true
                } else {
                    false
                }
            }
            192..=255 => {
                let nth: u64 = 1 << (value - 192);
                if nth & self.0[3] == 0 {
                    self.0[3] |= nth;
                    true
                } else {
                    false
                }
            }
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
