use criterion::{black_box, criterion_group, criterion_main, Criterion};

use primes::sieve_eratosthenes;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("eratosthenes 100_000", |b| b.iter(|| black_box(sieve_eratosthenes(1_301_789))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
