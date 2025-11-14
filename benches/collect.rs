use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::prelude::*;

fn collect_simplebitset(input: &Vec<u8>) -> Vec<u8> {
    let mut bs: simplebitset::BitSet = simplebitset::BitSet::new();
    for element in input {
        bs.insert(*element);
    }
    bs.into_iter().collect()
}

fn collect_hashset(input: &Vec<u8>) -> Vec<u8> {
    let mut bs: std::collections::HashSet<u8> = std::collections::HashSet::new();
    for element in input {
        bs.insert(*element);
    }
    bs.into_iter().collect()
}

fn collect_bit_set(input: &Vec<u8>) -> Vec<u8> {
    let mut bs: bit_set::BitSet<u32> = bit_set::BitSet::with_capacity(256);
    for element in input {
        bs.insert(*element as usize);
    }
    bs.into_iter().map(|x| x as u8).collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::rng();
    let mut input: Vec<u8> = Vec::new();

    for _ in 0..300 {
        input.push(rng.random::<u8>());
    }
    c.bench_with_input(
        BenchmarkId::new("collect bits simplebitset", "300 random numbers"),
        &input,
        |b, i| b.iter(|| collect_simplebitset(&i)),
    );
    c.bench_with_input(
        BenchmarkId::new("collect bits hashset", "300 random numbers"),
        &input,
        |b, i| b.iter(|| collect_hashset(&i)),
    );
    c.bench_with_input(
        BenchmarkId::new("collect bits bit_set", "300 random numbers"),
        &input,
        |b, i| b.iter(|| collect_bit_set(&i)),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
