use rustc_hash::FxHashMap;

const MAX_A_CACHE: usize = 64;

pub struct LegendrePrimeCounter {
    primes: Vec<usize>,

    pi_cache: FxHashMap<usize, usize>,
    phi_cache: FxHashMap<u64, usize>,
}

impl LegendrePrimeCounter {
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    #[must_use]
    pub fn new(limit: usize) -> Self {
        let primes = super::block_sieve(limit.isqrt());
        Self {
            primes,
            pi_cache: FxHashMap::default(),
            phi_cache: FxHashMap::default(),
        }
    }

    #[must_use]
    pub fn prime_factors(&self) -> &[usize] {
        &self.primes
    }

    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    #[must_use]
    pub fn count_primes(&mut self, n: usize) -> usize {
        if n < 2 {
            0
        } else if let Some(v) = self.pi_cache.get(&n) {
            *v
        } else {
            let a = self.count_primes(n.isqrt());
            let result = self.phi(n, a) + a - 1;
            self.pi_cache.insert(n, result);
            result
        }
    }

    #[inline(never)]
    fn phi(&mut self, x: usize, mut a: usize) -> usize {
        let mut result: isize = 0;

        loop {
            match a {
                0 => return (result + x.cast_signed()).cast_unsigned(),
                // remove multiples of 2
                1 => {
                    return (result + (x - (x >> 1)).cast_signed()).cast_unsigned();
                }
                // remove multiples of 2 and 3
                2 => {
                    let v = x - (x >> 1) - x / 3 + x / 6;
                    return (result + v.cast_signed()).cast_unsigned();
                }
                // remove multiples of 2,3,5
                3 => {
                    let positives = x + x / 6 + x / 10 + x / 15;
                    let negatives = (x >> 1) + x / 3 + x / 5 + x / 30;

                    let v = positives - negatives;
                    return (result + v.cast_signed()).cast_unsigned();
                }

                _ => {}
            }

            let pa = self.primes[a - 1];
            if x < pa {
                return (result + 1).cast_unsigned();
            }

            let key = phi_key(x, a);
            if a <= MAX_A_CACHE
                && let Some(v) = self.phi_cache.get(&key)
            {
                return (result + (*v).cast_signed()).cast_unsigned();
            }

            // recursively evaluate ONLY the smaller branch
            let rhs = self.phi(x / pa, a - 1);

            // accumulate:
            //
            // phi(x,a)
            //   = phi(x,a-1) - rhs
            //
            // so defer phi(x,a-1) into loop iteration

            result -= rhs.cast_signed();

            // continue iteratively down left spine
            a -= 1;
        }
    }
}

const fn phi_key(x: usize, a: usize) -> u64 {
    ((x as u64) << 16) | (a as u64)
}

#[cfg(test)]
mod tests {
    use crate::{counting::*, upper_bound_for_nth_prime};
    use rstest::rstest;

    #[rstest]
    #[case(1, 0)]
    #[case(10, 4)]
    #[case(100, 25)]
    #[case(1_000, 168)]
    #[case(10_000, 1_229)]
    #[case(100_000, 9_592)]
    #[case(1_000_000, 78_498)]
    #[case(10_000_000, 664_579)]
    #[case(100_000_000, 5_761_455)]
    #[case(1_000_000_000, 50_847_534)]
    fn test_prime_count(#[case] n: usize, #[case] expected_count: usize) {
        let upper_bound = upper_bound_for_nth_prime(n);
        let mut counter = LegendrePrimeCounter::new(upper_bound);
        assert_eq!(counter.count_primes(n), expected_count);
    }
}
