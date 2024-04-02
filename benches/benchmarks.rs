use criterion::{black_box, criterion_group, criterion_main, Criterion};

use primes::{block_sieve, nth_prime, sieve_eratosthenes};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("search 10_001", |b| b.iter(|| nth_prime(black_box(10_001))));
    c.bench_function("search 100_001", |b| b.iter(|| nth_prime(black_box(100_001))));
    c.bench_function("search 1_000_001", |b| b.iter(|| nth_prime(black_box(1_000_001))));
    c.bench_function("eratosthenes 100_000", |b| b.iter(|| sieve_eratosthenes(black_box(1_301_789))));
    c.bench_function("eratosthenes 1_000_000", |b| b.iter(|| sieve_eratosthenes(black_box(179_595_382))));
    c.bench_function("block 100_000", |b| b.iter(|| block_sieve(black_box(1_301_789))));
    c.bench_function("block 1_000_000", |b| b.iter(|| block_sieve(black_box(179_595_382))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
