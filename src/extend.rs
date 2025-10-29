use crate::bitset::BitSet;

impl std::iter::Extend<u8> for BitSet {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        for elem in iter {
            self.insert(elem);
        }
    }
}
