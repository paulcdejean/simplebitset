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

    /// Returns true if the set contains a value.
    pub fn contains_v2(&self, value: u8) -> bool {
        let index: u8 = value / 64;
        let offset: u8 = value % 64;
        let num: u64 = self.0[usize::from(index)];
        let mask: u64 = 1 << offset;
        (num & mask) != 0
    }
}
