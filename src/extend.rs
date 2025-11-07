use crate::bitset::BitSet;

impl core::iter::Extend<u8> for BitSet {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        for elem in iter {
            self.insert(elem);
        }
    }
}

impl<'a> core::iter::Extend<&'a u8> for BitSet {
    fn extend<T: IntoIterator<Item = &'a u8>>(&mut self, iter: T) {
        self.extend(iter.into_iter().cloned())
    }
}
