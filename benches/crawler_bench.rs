use criterion::{criterion_group, criterion_main, Criterion};
use crawler_challenge::example::crawler;



fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Crawler", |b| b.iter(|| crawler()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);