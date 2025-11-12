# Simple BitSet

A dead simple bitset. Not generic at all, it only stores u8s and nothing else. For its underlying storage it uses a `[usize; 4]`

Attempts to match the interface of `std::collections::HashSet` with some exceptions.

### HashSet methods not implemented for simple BitSet

* `get_or_insert_with`: Low priority TODO.
* `entry`: Medium priority TODO.
* `hasher`: Won't be implemented. No sane way to implement this.

### HashSet methods that are behind the `compatibility-methods` feature

* `capacity`: Just returns `256usize`.
* `reserve`: Does nothing.
* `shrink_to`: Does nothing.
* `shrink_to_fit`: Does nothing.
* `try_reserve`: Returns `Ok(())`.
* `with_capacity`: Same behavior as `new()` just returns an empty BitSet.
* `with_hasher`: Same behavior as `new()` just returns an empty BitSet. Note that hasher parameter can be any type.
* `with_capacity_and_hasher`: Same behavior as `new()` just returns an empty bitset. Note that hasher parameter can be any type.
