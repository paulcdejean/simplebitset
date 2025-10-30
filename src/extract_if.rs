use crate::bitset::BitSet;

/// A draining, filtering iterator over the items of a BitSet.
/// This struct is created by the extract_if method on BitSet.
pub struct ExtractIf<'a, F> {
    bitset: &'a mut BitSet,
    f: F,
    indices: core::ops::RangeInclusive<u8>,
}

impl<'a, F: FnMut(u8) -> bool> Iterator for ExtractIf<'a, F> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let next = self
            .indices
            .find(|&x| self.bitset.get(x).is_some_and(&mut self.f))?;
        self.bitset.remove(next);
        Some(next)
    }
}

impl BitSet {
    /// Creates an iterator which uses a closure to determine if an element should be removed.
    /// If the closure returns true, the element is removed from the set and yielded. If the closure returns false, or panics, the element remains in the set and will not be yielded.
    /// If the returned ExtractIf is not exhausted, e.g. because it is dropped without iterating or the iteration short-circuits, then the remaining elements will be retained. Use retain with a negated predicate if you do not need the returned iterator.
    pub fn extract_if<F>(&mut self, pred: F) -> ExtractIf<'_, F> {
        ExtractIf {
            bitset: self,
            f: pred,
            indices: core::ops::RangeInclusive::new(0, u8::MAX),
        }
    }
}
