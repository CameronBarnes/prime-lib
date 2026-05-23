use std::hint::black_box;

fn main() {
    let primes = black_box(primes::block_sieve(primes::upper_bound_for_nth_prime(1_000_000_000)));
    println!("len: {}, val: {}", primes.len(), primes[primes.len() - 1]);
}
