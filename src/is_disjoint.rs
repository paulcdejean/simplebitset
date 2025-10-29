use crate::bitset::BitSet;

impl BitSet {
    /// Returns true if self has no elements in common with other. This is equivalent to checking for an empty intersection.
    pub fn is_disjoint(&self, other: &Self) -> bool {
        (self & other).is_empty()
    }
}
