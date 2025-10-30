//! # Simple BitSet
//! Basically a `HashSet<u8>` but faster. Even though for a HashSet operations run in O(1) time, for a T that has a limited number of possible
//! values, there's faster approaches than hashing. A BitSet works by storing each element in a single bit, then using very fast bitwise
//! operations in order to perform set functions on those bits.
//!
//! Simple BitSet is an implementation of BitSet that is purposefully not generic. It only stores u8s. This means less and easier to read
//! code, and it allows optimization work to be more focused.
//!
//! If your use case involves some type that can be represented internally in a single byte, such as a enum with Repr u8, or ASCII chars,
//! and you wish to use Simple BitSet, then I'd recommend you make a Newtype.
//!
//! If you need to store a type that has more than 256 possible values, this crate isn't a good match for that use case.
//!
//! ### Features
//! * `copy` - Enabled by default. Causes BitSet to implement the Copy trait.

mod bitset;
mod contains;
mod debug;
mod difference;
mod drain;
mod extend;
mod extract_if;
mod from_iter;
mod get;
mod insert;
mod intersection;
mod into_iter;
mod is_disjoint;
mod is_empty;
mod is_subset;
mod is_superset;
mod iter;
mod len;
mod memes;
mod new;
mod remove;
mod replace;
mod retain;
mod symmetric_difference;
mod take;
mod union;

pub use bitset::BitSet;
pub use into_iter::IntoIter;
pub use iter::Iter;
