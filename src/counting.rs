

pub struct LegendrePrimeCounter {
    primes: Vec<usize>,
}

impl LegendrePrimeCounter {
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    #[must_use]
    pub fn new(limit: usize) -> Self {
        let primes = super::block_sieve((limit as f64).sqrt() as usize);
        Self { primes }
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
    pub fn count_primes(&self, n: usize) -> usize {
        if n < 2 {
            0
        } else {
            let a = self.count_primes((n as f64).sqrt() as usize);
            self.phi(n, a) + a - 1
        }
    }

    fn phi(&self, x: usize, a: usize) -> usize {
        match a {
            0 => x,
            1 => x - (x >> 1),
            a => {
                let pa = self.primes[a - 1];
                if x < pa {
                    1
                } else {
                    self.phi(x, a - 1) - self.phi(x / pa, a - 1)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{upper_bound_for_nth_prime, counting::*};
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
        let counter = LegendrePrimeCounter::new(upper_bound);
        assert_eq!(counter.count_primes(n), expected_count);
    }
}
