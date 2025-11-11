use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::prelude::*;
use simplebitset::BitSet;

fn insert_vec_branchless(input: &Vec<u8>) -> BitSet {
    let mut bs: BitSet = BitSet::new();
    for element in input {
        bs.insert(*element);
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
        BenchmarkId::new("insert rand vec branchless", "200 random numbers"),
        &input,
        |b, i| b.iter(|| insert_vec_branchless(&i)),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
