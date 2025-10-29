use crate::bitset::BitSet;

impl BitSet {
    /// Returns true if the set is a superset of another, i.e., self contains at least all the values in other.
    pub fn is_superset(&self, other: &Self) -> bool {
        (other - self).is_empty()
    }
}
