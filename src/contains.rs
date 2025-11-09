use crate::bitset::BitSet;

impl BitSet {
    /// Returns true if the set contains a value.
    pub fn contains(&self, value: u8) -> bool {
        let index: u8 = value / 64;
        let offset: u8 = value % 64;
        // SAFETY: a u8 divided by 64 is between 0 and 3. MIR doesn't know this though.
        let num: u64 = unsafe {
            *self.0.get_unchecked(index as usize)
        };
        let mask: u64 = 1 << offset;
        (num & mask) != 0
    }
}
