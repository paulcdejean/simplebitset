/// A simple BitSet is like a HashMap<u8> except much much faster. The interface is very similar to HashMap.
/// Whether or not it implements `Copy` can be toggled with a feature.
#[derive(PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "copy", derive(Copy))]
pub struct BitSet(pub(crate) [u64; 4]);
