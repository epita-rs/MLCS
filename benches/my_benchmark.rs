use criterion::{black_box, criterion_group, criterion_main, Criterion};
use testsuite::astar;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("astar 20", |b| b.iter(|| astar(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
