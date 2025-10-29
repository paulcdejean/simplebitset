use crate::bitset::BitSet;

impl std::iter::FromIterator<u8> for BitSet {
    /// Creates a value from an iterator.
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut result = BitSet::new();
        for i in iter {
            result.insert(i);
        }
        result
    }
}
