use crate::bitset::BitSet;
use std::ops::BitOr;

impl BitSet {
    /// Returns a new bitset that is the union of self and other.
    pub fn union(&self, other: &Self) -> Self {
        BitSet(std::array::from_fn(|i| self.0[i] | other.0[i]))
    }
}

impl BitOr for BitSet {
    type Output = Self;

    /// Returns a new bitset that is the union of self and other.
    fn bitor(self, rhs: Self) -> Self::Output {
        BitSet(std::array::from_fn(|i| self.0[i] | rhs.0[i]))
    }
}

impl BitOr for &BitSet {
    type Output = BitSet;

    /// Returns a new bitset that is the union of self and other.
    fn bitor(self, rhs: Self) -> Self::Output {
        BitSet(std::array::from_fn(|i| self.0[i] | rhs.0[i]))
    }
}
