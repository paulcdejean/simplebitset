use crate::bitset::BitSet;

pub struct IntoIter([u64; 4]);

impl Iterator for IntoIter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..self.0.len() {
            if self.0[i] != 0 {
                let trailing_zeros: u8 = self.0[i].trailing_zeros() as u8;
                self.0[i] &= self.0[i] - 1u64;
                return Some(trailing_zeros);
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
