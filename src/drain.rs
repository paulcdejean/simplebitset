// If this is wrong please let me know.

use crate::bitset::BitSet;

/// A draining iterator over the items of a BitSet.
pub type Drain = crate::into_iter::IntoIter;

impl BitSet {
    /// Clears the set, returning all elements as an iterator.
    pub fn drain(&mut self) -> Drain {
        std::mem::take(self).into_iter()
    }
}
