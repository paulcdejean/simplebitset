use crate::bitset::BitSet;

/// A borrowing iterator over the items of a BitSet.
pub struct Iter<'a> {
    bitset: &'a BitSet,
    indices: core::ops::RangeInclusive<u8>,
}

impl Iterator for Iter<'_> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.indices.by_ref().find(|&x| self.bitset.contains(x))
    }
}

impl BitSet {
    /// An iterator visiting all elements in ascending order.
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            bitset: self,
            indices: core::ops::RangeInclusive::new(0, u8::MAX),
        }
    }
}

impl<'a> IntoIterator for &'a BitSet {
    type Item = u8;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrow_iter() {
        let example: BitSet = BitSet::total_set();
        let mut n: usize = 0;
        for bit in example.iter() {
            assert_eq!(bit as usize, n);
            n += 1;
        }
        assert_eq!(n, 256);
    }
}
