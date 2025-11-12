use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::prelude::*;
use simplebitset::BitSet;

fn collect_bitset(input: &Vec<u8>) -> Vec<u8> {
    let mut bs: BitSet = BitSet::new();
    for element in input {
        bs.insert(*element);
    }
    bs.iter().collect()
}

fn collect_hashset(input: &Vec<u8>) -> Vec<u8> {
    let mut bs: std::collections::HashSet<u8> = std::collections::HashSet::new();
    for element in input {
        bs.insert(*element);
    }
    bs.iter().map(|x| *x).collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::rng();
    let mut input: Vec<u8> = Vec::new();

    for _ in 0..100 {
        input.push(rng.random::<u8>());
    }
    c.bench_with_input(
        BenchmarkId::new("collect bits bitset", "100 random numbers"),
        &input,
        |b, i| b.iter(|| collect_bitset(&i)),
    );
    c.bench_with_input(
        BenchmarkId::new("collect bits hashset", "100 random numbers"),
        &input,
        |b, i| b.iter(|| collect_hashset(&i)),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
