use crate::bitset::BitSet;

impl BitSet {
    /// Retains only the elements specified by the predicate.
    /// In other words, remove all elements e for which f(&e) returns false. The elements are visited in unsorted (and unspecified) order.
    pub fn retain<F: FnMut(u8) -> bool>(&mut self, mut f: F) {
        let iter_set: BitSet = *self;
        for bit in iter_set {
            if !f(bit) {
                self.remove(bit);
            }
        }
    }
}
