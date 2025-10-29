# Simple BitSet

A dead simple bitset. Not generic at all, it only stores u8s and nothing else. For its underlying storage it uses a `[u64; 4]`

Attempts to match the interface of `std::collections::HashSet` with some exceptions.

### HashSet methods not implemented for simple BitSet

* extract_if: I should have implemented this but it was late and I was getting tired.
* drain: The spec of this is a headache. I can try to add this if someone requests it.
* get_or_insert_with: The sepc of this is a headache. I can try to add this if someone requests it.
* Extend<&'a u8>: This shouldn't be needed because u8 is copy. Also I was getting a lifetime headache.
* entry: Unneeded becuase u8 is Copy.
* hasher: Nonsensical since this is not a hash set.
* reserve: Nonsensical since the bitset has a fixed size of 256.
* shrink_to: Nonsensical since the bitset has a fixed size of 256.
* shrink_to_fit: Nonsensical since the bitset has a fixed size of 256.
* try_reserve: Nonsensical since the bitset has a fixed size of 256.
* with_capacity: Nonsensical since the bitset has a fixed size of 256.
* with_hasher: Nonsensical since this is not a hash set.
* with_capacity_and_hasher: See above.
