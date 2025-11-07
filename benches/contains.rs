use criterion::{Criterion, criterion_group, criterion_main};
use rand::prelude::*;
use simplebitset::BitSet;

fn contains_hundred(input: &BitSet) -> [bool; 100] {
    let mut result: [bool; 100] = [false; 100];
    for i in 0u8..100u8 {
        result[i as usize] = input.contains(i);
    }
    result
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::rng();
    let mut input: BitSet = BitSet::new();
    for _ in 0..200 {
        input.insert(rng.random::<u8>());
    }
    c.bench_function("first hundred contains", |b| {
        b.iter(|| contains_hundred(&input))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
