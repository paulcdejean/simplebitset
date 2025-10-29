use crate::bitset::BitSet;

impl BitSet {
    /// Adds a value to the set, replacing the existing value, if any, that is equal to the given one. Returns the replaced value.
    pub fn replace(&mut self, value: u8) -> Option<u8> {
        match value {
            0..=63 => {
                let nth: u64 = 1 << value;
                if nth & self.0[0] == 0 {
                    self.0[0] |= nth;
                    Some(value)
                } else {
                    None
                }
            }
            64..=127 => {
                let nth: u64 = 1 << (value - 64);
                if nth & self.0[1] == 0 {
                    self.0[1] |= nth;
                    Some(value)
                } else {
                    None
                }
            }
            128..=191 => {
                let nth: u64 = 1 << (value - 128);
                if nth & self.0[2] == 0 {
                    self.0[2] |= nth;
                    Some(value)
                } else {
                    None
                }
            }
            192..=255 => {
                let nth: u64 = 1 << (value - 192);
                if nth & self.0[3] == 0 {
                    self.0[3] |= nth;
                    Some(value)
                } else {
                    None
                }
            }
        }
    }
}
