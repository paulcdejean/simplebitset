use crate::bitset::BitSet;

impl BitSet {
    /// Returns true if the set contains a value.
    pub fn contains(&self, value: u8) -> bool {
        match value {
            0..=63 => {
                let nth: u64 = 1 << value;
                nth & self.0[0] != 0
            }
            64..=127 => {
                let nth: u64 = 1 << (value - 64);
                nth & self.0[1] != 0
            }
            128..=191 => {
                let nth: u64 = 1 << (value - 128);
                nth & self.0[2] != 0
            }
            192..=255 => {
                let nth: u64 = 1 << (value - 192);
                nth & self.0[3] != 0
            }
        }
    }
}
