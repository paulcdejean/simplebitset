use crate::bitset::BitSet;
use std::ops::BitXor;

impl BitSet {
    /// Returns a new bitset that is the intersection of self and other.
    pub fn symmetric_difference(&self, other: &Self) -> Self {
        BitSet(std::array::from_fn(|i| self.0[i] ^ other.0[i]))
    }
}

impl BitXor for BitSet {
    type Output = Self;

    /// Returns a new bitset that is the intersection of self and other.
    fn bitxor(self, rhs: Self) -> Self::Output {
        BitSet(std::array::from_fn(|i| self.0[i] ^ rhs.0[i]))
    }
}

impl BitXor for &BitSet {
    type Output = BitSet;

    /// Returns a new bitset that is the intersection of self and other.
    fn bitxor(self, rhs: Self) -> Self::Output {
        BitSet(std::array::from_fn(|i| self.0[i] ^ rhs.0[i]))
    }
}
