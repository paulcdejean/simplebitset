use crate::bitset::BitSet;

impl BitSet {
    /// Adds a value to the set.
    ///
    /// Returns whether the value was newly inserted. That is:
    ///
    /// - If the set did not previously contain this value, `true` is returned.
    /// - If the set already contained this value, `false` is returned.
    pub fn insert(&mut self, value: u8) -> bool {
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
