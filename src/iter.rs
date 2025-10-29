use crate::bitset::BitSet;

pub struct Iter<'a> {
    bitset: &'a BitSet,
    index: u8,
}

impl Iterator for Iter<'_> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < u8::MAX {
            if self.bitset.contains(self.index) {
                let result: u8 = self.index;
                self.index += 1;
                return Some(result);
            } else {
                self.index += 1;
            }
        }
        None
    }
}

impl BitSet {
    /// An iterator visiting all elements in ascending order. 
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            bitset: self,
            index: 0,
        }
    }
}

impl<'a> IntoIterator for &'a BitSet {
    type Item = u8;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
