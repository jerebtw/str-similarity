use criterion::{criterion_group, criterion_main, Criterion};
use str_similarity::{compare_two_strings, find_best_match};

pub fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("compares Nacht and Nacht", |b| {
    b.iter(|| compare_two_strings("Nacht", "Night"))
  });
  c.bench_function("compares Night and Nacht", |b| {
    b.iter(|| compare_two_strings("Nacht", "Nacht"))
  });
  c.bench_function("compares Night Night Night and Nacht Nacht Nacht", |b| {
    b.iter(|| compare_two_strings("Night Night Night", "Nacht Nacht Nacht"))
  });
  c.bench_function("compares Night Night Night and Nacht Nacht Nacht", |b| {
    b.iter(|| compare_two_strings("Night Night Night", "Nacht Nacht Nacht"))
  });
  c.bench_function("find_best_match Night and [Nacht, Night, Nacht]", |b| {
    b.iter(|| find_best_match("Night", vec!["Nacht", "Night", "Nacht"]))
  });
  c.bench_function(
    "find_best_match Night Night Night and [Nacht Nacht Nacht, Night Night Night, Night]",
    |b| {
      b.iter(|| {
        find_best_match(
          "Night Night Night",
          vec!["Nacht Nacht Nacht", "Night Night Night", "Night"],
        )
      })
    },
  );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
