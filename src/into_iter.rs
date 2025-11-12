use crate::bitset::BitSet;
use crate::bitset::BITSET_ARRAY_SIZE;

/// A consuming iterator over the items of a BitSet.
pub struct IntoIter([usize; BITSET_ARRAY_SIZE]);

impl Iterator for IntoIter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..self.0.len() {
            if self.0[i] != 0 {
                let trailing_zeros: u8 = self.0[i].trailing_zeros() as u8;
                self.0[i] &= self.0[i] - 1usize;
                return Some(trailing_zeros + i as u8 * 64);
            }
        }
        None
    }
}

impl IntoIterator for BitSet {
    type Item = u8;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let example: BitSet = BitSet::total_set();
        let mut n: usize = 0;
        for bit in example {
            assert_eq!(bit as usize, n);
            n += 1;
        }
        assert_eq!(n, 256);
    }
}
