use crate::bitset::BitSet;

impl BitSet {
    /// Removes and returns the value in the set, if any, that is equal to the given one.
    pub fn take(&mut self, value: u8) -> Option<u8> {
        match value {
            0..=63 => {
                let nth: u64 = 1 << value;
                if nth & self.0[0] == 0 {
                    None
                } else {
                    self.0[0] &= !nth;
                    Some(value)
                }
            }
            64..=127 => {
                let nth: u64 = 1 << (value - 64);
                if nth & self.0[1] == 0 {
                    None
                } else {
                    self.0[1] &= !nth;
                    Some(value)
                }
            }
            128..=191 => {
                let nth: u64 = 1 << (value - 128);
                if nth & self.0[2] == 0 {
                    None
                } else {
                    self.0[2] &= !nth;
                    Some(value)
                }
            }
            192..=255 => {
                let nth: u64 = 1 << (value - 192);
                if nth & self.0[3] == 0 {
                    None
                } else {
                    self.0[3] &= !nth;
                    Some(value)
                }
            }
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
