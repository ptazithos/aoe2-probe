use aoe2_probe::Scenario;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Parsing AoE2 Scenario", |b| {
        b.iter(|| Scenario::from_file(black_box("./resources/chapter_1.aoe2scenario")))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(30));
    targets = criterion_benchmark
);
criterion_main!(benches);
