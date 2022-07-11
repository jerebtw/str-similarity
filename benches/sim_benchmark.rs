use criterion::{black_box, criterion_group, criterion_main, Criterion};
use str_similarity::compare_two_strings;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("compares Nacht and Nacht", |b| {
        b.iter(|| compare_two_strings("Nacht", "Night"))
    });
    c.bench_function("compares Night and Nacht", |b| {
        b.iter(|| compare_two_strings(black_box("Nacht"), black_box("Nacht")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
