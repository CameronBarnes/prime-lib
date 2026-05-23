use std::hint::black_box;

fn main() {
    println!("{}", black_box(primes::nth_prime(50_000_000_000)));
}
