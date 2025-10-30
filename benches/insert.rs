use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use simplebitset::BitSet;

fn many_inserts() {
    let mut bs: BitSet = black_box(BitSet::new());
    for _ in 0..1_00 {
        for z in 0..255 {
            black_box(bs.insert(black_box(z)));
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("many inserts", |b| b.iter(|| many_inserts()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
