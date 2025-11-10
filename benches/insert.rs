use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::prelude::*;

#[derive(PartialEq, Eq, Clone, Hash, Copy)]
pub struct BitSet(pub(crate) [u64; 4]);

impl BitSet {
    pub fn new() -> Self {
        Self([0; 4])
    }
    pub fn insert_blind_branching(&mut self, value: u8) {
        match value {
            0..=63 => {
                let nth: u64 = 1 << value;
                self.0[0] |= nth;
            }
            64..=127 => {
                let nth: u64 = 1 << (value - 64);
                self.0[1] |= nth;
            }
            128..=191 => {
                let nth: u64 = 1 << (value - 128);
                self.0[2] |= nth;
            }
            192..=255 => {
                let nth: u64 = 1 << (value - 192);
                self.0[3] |= nth;
            }
        }
    }
    pub fn insert_blind_branchless(&mut self, value: u8) {
        let index: u8 = value / 64;
        let offset: u8 = value % 64;
        // SAFETY: a u8 divided by 64 is between 0 and 3. MIR doesn't know this though.
        let num: &mut u64 = unsafe { self.0.get_unchecked_mut(index as usize) };
        let mask: u64 = 1 << offset;
        *num |= mask;
    }
}

fn insert_vec_branching(input: &Vec<u8>) -> BitSet {
    let mut bs: BitSet = BitSet::new();
    for element in input {
        bs.insert_blind_branching(*element);
    }
    bs
}

fn insert_vec_branchless(input: &Vec<u8>) -> BitSet {
    let mut bs: BitSet = BitSet::new();
    for element in input {
        bs.insert_blind_branchless(*element);
    }
    bs
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::rng();
    let mut input: Vec<u8> = Vec::new();

    for _ in 0..300 {
        input.push(rng.random::<u8>());
    }

    c.bench_with_input(
        BenchmarkId::new("insert rand vec branching", "200 random numbers"),
        &input,
        |b, i| {
            b.iter(|| insert_vec_branching(&i))
        },
    );
    c.bench_with_input(
        BenchmarkId::new("insert rand vec branchless", "200 random numbers"),
        &input,
        |b, i| {
            b.iter(|| insert_vec_branchless(&i))
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
