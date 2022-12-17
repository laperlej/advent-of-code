use criterion::{criterion_group, criterion_main, Criterion};

#[path = "../examples/aoc_2022_1.rs"]
mod aoc_2022_1;



fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("functional", |b| b.iter(|| aoc_2022_1::solve_aoc_functional()));
    c.bench_function("procedural", |b| b.iter(|| aoc_2022_1::solve_aoc()));
    c.bench_function("parallel", |b| b.iter(|| aoc_2022_1::solve_aoc_parallel()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
