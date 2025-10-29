use crate::bitset::BitSet;

impl BitSet {
    /// Returns true if the set is a subset of another, i.e., other contains at least all the values in self.
    pub fn is_subset(&self, other: &Self) -> bool {
        (self - other).is_empty()
    }
}
