use crate::bitset::BitSet;

impl BitSet {
    /// Removes a value from the set. Returns whether the value was present in the set.
    pub fn remove(&mut self, value: u8) -> bool {
        let index: u8 = value / 64;
        let offset: u8 = value % 64;
        // SAFETY: a u8 divided by 64 is between 0 and 3. MIR doesn't know this though.
        let num: &mut u64 = unsafe { self.0.get_unchecked_mut(index as usize) };
        let mask: u64 = 1 << offset;
        if *num & mask == 0 {
            false
        } else {
            *num &= !mask;
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_all_from_totalset() {
        let mut example: BitSet = BitSet::total_set();
        for n in 0u8..=255u8 {
            assert!(example.contains(n));
            assert!(example.remove(n));
            assert!(!example.remove(n));
            assert!(!example.contains(n));
        }
    }
}
