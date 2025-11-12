use crate::bitset::BitSet;

impl BitSet {
    /// Returns true if the set contains a value.
    #[inline] // Required because MIR doesn't know u8::MAX / 4 is between 0 and 3.
    pub fn contains(&self, value: u8) -> bool {
        let index: u8 = value / 64;
        let offset: u8 = value % 64;
        let num: u64 = self.0[index as usize];
        let mask: u64 = 1 << offset;
        (num & mask) != 0
    }
}
