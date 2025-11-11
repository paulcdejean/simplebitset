use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::prelude::*;
use simplebitset::BitSet;

fn remove_vec_branching(input: &Vec<u8>) -> BitSet {
    let mut bs: BitSet = BitSet::total_set();
    for element in input {
        bs.remove(*element);
    }
    bs
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::rng();
    let mut input: Vec<u8> = Vec::new();

    for _ in 0..200 {
        input.push(rng.random::<u8>());
    }
    c.bench_with_input(
        BenchmarkId::new("remove_branching", "200 random numbers"),
        &input,
        |b, i| b.iter(|| remove_vec_branching(&i)),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
