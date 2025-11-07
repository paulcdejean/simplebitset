use criterion::{Criterion, criterion_group, criterion_main};
use core::hint::black_box;

use simplebitset::BitSet;

use rand::prelude::*;

fn many_inserts() {
    let mut bs: BitSet = black_box(BitSet::new());
    for _ in 0..1_00 {
        for z in 0..255 {
            black_box(black_box(&mut bs).insert(black_box(z)));
        }
    }
}

fn insert_vec(input: &Vec<u8>) {
    let mut bs: BitSet = black_box(BitSet::new());
    for element in input {
        black_box(black_box(&mut bs).insert(black_box(*element)));
    }
}

fn double_insert() {
    let mut bs: BitSet = black_box(BitSet::new());
    for n in 0..=255 {
        black_box(black_box(&mut bs).insert(black_box(n)));
        black_box(black_box(&mut bs).insert(black_box(n)));
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::rng();

    let mut input: Vec<u8> = Vec::new();

    for _ in 0..100 {
        input.push(rng.random::<u8>());
    }

    c.bench_function("insert rand vec", |b| b.iter(|| insert_vec(&input)));
    c.bench_function("many inserts", |b| b.iter(|| many_inserts()));
    c.bench_function("double insert", |b| b.iter(|| double_insert()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
