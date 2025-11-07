use crate::bitset::BitSet;

use core::ops::Sub;

impl BitSet {
    /// Returns the difference of self and other as a new BitSet.
    pub fn difference(&self, other: &Self) -> Self {
        BitSet(core::array::from_fn(|i| self.0[i] & !other.0[i]))
    }
}

impl Sub for BitSet {
    type Output = Self;

    /// Returns the difference of self and rhs as a new BitSet.
    fn sub(self, rhs: Self) -> Self::Output {
        BitSet(core::array::from_fn(|i| self.0[i] & !rhs.0[i]))
    }
}

impl Sub for &BitSet {
    type Output = BitSet;

    /// Returns the difference of self and rhs as a new BitSet.
    fn sub(self, rhs: Self) -> Self::Output {
        BitSet(core::array::from_fn(|i| self.0[i] & !rhs.0[i]))
    }
}
