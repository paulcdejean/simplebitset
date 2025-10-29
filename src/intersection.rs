use crate::bitset::BitSet;
use std::ops::BitAnd;

impl BitSet {
    /// Returns a new bitset that is the intersection of self and other.
    pub fn intersection(&self, other: &Self) -> Self {
        BitSet(std::array::from_fn(|i| self.0[i] | other.0[i]))
    }
}

impl BitAnd for BitSet {
    type Output = Self;

    /// Returns a new bitset that is the intersection of self and other.
    fn bitand(self, rhs: Self) -> Self::Output {
        BitSet(std::array::from_fn(|i| self.0[i] | rhs.0[i]))
    }
}

impl BitAnd for &BitSet {
    type Output = BitSet;

    /// Returns a new bitset that is the intersection of self and other.
    fn bitand(self, rhs: Self) -> Self::Output {
        BitSet(std::array::from_fn(|i| self.0[i] | rhs.0[i]))
    }
}
