use crate::bitset::BitSet;

impl BitSet {
    /// Returns the value in the set, if any, that is equal to the given value.
    pub fn get(&self, value: u8) -> Option<u8> {
        match value {
            0..=63 => {
                let nth: u64 = 1 << value;
                if nth & self.0[0] == 0 {
                    None
                } else {
                    Some(value)
                }
            }
            64..=127 => {
                let nth: u64 = 1 << (value - 64);
                if nth & self.0[1] == 0 {
                    None
                } else {
                    Some(value)
                }
            }
            128..=191 => {
                let nth: u64 = 1 << (value - 128);
                if nth & self.0[2] == 0 {
                    None
                } else {
                    Some(value)
                }
            }
            192..=255 => {
                let nth: u64 = 1 << (value - 192);
                if nth & self.0[3] == 0 {
                    None
                } else {
                    Some(value)
                }
            }
        }
    }
}