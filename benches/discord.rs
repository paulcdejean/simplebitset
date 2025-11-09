use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::Rng;

use simplebitset::BitSet;

pub fn contains_hundred_ordered_1(input: &BitSet) -> [bool; 100] {
    let mut result: [bool; 100] = [false; 100];
    for i in 0u8..100u8 {
        result[i as usize] = input.contains(i);
    }
    result
}

pub fn contains_hundred_ordered_2(input: &BitSet) -> [bool; 100] {
    let mut result: [bool; 100] = [false; 100];
    for i in 0u8..100u8 {
        result[i as usize] = input.contains_v2(i);
    }
    result
}

pub fn contains_hundred_random_1(input: &BitSet, values: &[u8; 100]) -> [bool; 100] {
    let mut result: [bool; 100] = [false; 100];
    for i in 0usize..100usize {
        result[i] = input.contains(values[i]);
    }
    result
}

pub fn contains_hundred_random_2(input: &BitSet, values: &[u8; 100]) -> [bool; 100] {
    let mut result: [bool; 100] = [false; 100];
    for i in 0usize..100usize {
        result[i] = input.contains_v2(values[i]);
    }
    result
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("discord");
    let mut rng = rand::rng();
    let mut input: BitSet = BitSet::new();
    for _ in 0..200 {
        input.insert(rng.random::<u8>());
    }
    group.bench_with_input(BenchmarkId::new("branch_ordered", ""), &input, |b, i| {
        b.iter(|| contains_hundred_ordered_1(&i));
    });
    group.bench_with_input(BenchmarkId::new("branchless_ordered", ""), &input, |b, i| {
        b.iter(|| contains_hundred_ordered_2(&i));
    });
    let mut values: [u8; 100] = [0; 100];
    for i in 0..values.len() {
        values[i] = rng.random::<u8>();
    }
    group.bench_with_input(
        BenchmarkId::new("branch_random", ""),
        &(&input, &values),
        |b, (input, values)| {
            b.iter(|| contains_hundred_random_1(input, values));
        },
    );
    group.bench_with_input(
        BenchmarkId::new("branchless_random", ""),
        &(&input, &values),
        |b, (input, values)| {
            b.iter(|| contains_hundred_random_2(input, values));
        },
    );
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
