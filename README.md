# Simple BitSet

A dead simple bitset. Not generic at all, it only stores u8s and nothing else. For its underlying storage it uses 256 bits.

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

### FAQ

* **Q:** How does this compare to `HashSet<u8>`?
* **A:** Roughly 8x faster with an identical interface.
* **Q:** How does this compare to the `bit_set` crate?
* **A:** It seems simplebitset is maybe 30% faster than the `bit_set` crate. However `bit_set` can store up to usize bits, while simplebitset only stores 256 bits.
* **Q:** How does this compare to the `bitvec` crate?
* **A:** The `bitvec` create has a completely different interface from simplebitset's interface. This makes comparison challenging. Feel free to run your own comparisons though and let me know how it goes via github issue or something.
