use criterion::{Criterion, criterion_group, criterion_main};
use rand::prelude::*;
use simplebitset::BitSet;
use std::hint::black_box;

fn contains_hundred_ordered(input: &BitSet) -> [bool; 100] {
    let mut result: [bool; 100] = [false; 100];
    for i in 0u8..100u8 {
        result[i as usize] = input.contains(i);
    }
    result
}

fn contains_hundred_random(input: &BitSet, values: &[u8; 100]) -> [bool; 100] {
    let mut result: [bool; 100] = [false; 100];
    for i in 0usize..100usize {
        result[i] = input.contains(values[i]);
    }
    result
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::rng();
    let mut input: BitSet = BitSet::new();
    for _ in 0..200 {
        input.insert(rng.random::<u8>());
    }
    c.bench_function("hundred contains ordered", |b| {
        b.iter(|| contains_hundred_ordered(&input))
    });
    let mut values: [u8; 100] = [0; 100];
    for i in 0..values.len() {
        values[i] = rng.random::<u8>();
    }
    c.bench_function("hundred contains random", |b| {
        b.iter(|| contains_hundred_random(&input, &values))
    });

    let single = rng.random::<u8>();
    c.bench_function("single contains", |b| {
        b.iter(|| black_box(input).contains(black_box(single)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
