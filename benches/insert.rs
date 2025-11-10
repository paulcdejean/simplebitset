use core::hint::black_box;
use criterion::{Criterion, criterion_group, criterion_main};

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

fn many_inserts_v2() {
    let mut bs: BitSet = black_box(BitSet::new());
    for _ in 0..1_00 {
        for z in 0..255 {
            black_box(black_box(&mut bs).insert_v2(black_box(z)));
        }
    }
}

fn insert_vec_v2(input: &Vec<u8>) {
    let mut bs: BitSet = black_box(BitSet::new());
    for element in input {
        black_box(black_box(&mut bs).insert_v2(black_box(*element)));
    }
}

fn double_insert_v2() {
    let mut bs: BitSet = black_box(BitSet::new());
    for n in 0..=255 {
        black_box(black_box(&mut bs).insert_v2(black_box(n)));
        black_box(black_box(&mut bs).insert_v2(black_box(n)));
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::rng();

    let mut input: Vec<u8> = Vec::new();

    for _ in 0..200 {
        input.push(rng.random::<u8>());
    }

    c.bench_function("insert rand vec v1", |b| b.iter(|| insert_vec(black_box(&input))));
    c.bench_function("many inserts v1", |b| b.iter(|| many_inserts()));
    c.bench_function("double insert v1", |b| b.iter(|| double_insert()));
    c.bench_function("insert rand vec v2", |b| b.iter(|| insert_vec_v2(black_box(&input))));
    c.bench_function("many inserts v2", |b| b.iter(|| many_inserts_v2()));
    c.bench_function("double insert v2", |b| b.iter(|| double_insert_v2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
