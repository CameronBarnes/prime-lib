pub mod counting;

use counting::LegendrePrimeCounter;

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]
#[must_use]
pub fn upper_bound_for_nth_prime(n: usize) -> usize {
    static LOW_PRIMES: [usize; 5] = [2, 3, 5, 7, 11];
    if n < 6 {
        LOW_PRIMES[n - 1]
    } else if n < 7022 {
        let n = n as f64;
        let log_n = n.ln();
        n.mul_add(log_n, n * log_n.ln()) as usize
    } else {
        let n = n as f64;
        let log_n = n.ln();
        n.mul_add(log_n, n * (log_n.ln() - 0.9385)) as usize
    }
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]
#[must_use]
pub fn lower_bound_for_nth_prime(n: usize) -> usize {
    static LOW_PRIMES: [usize; 5] = [2, 3, 5, 7, 11];
    if n < 6 {
        LOW_PRIMES[n - 1]
    } else {
        let n = n as f64;
        let log_n = n.ln();
        (n * (log_n + log_n.ln() - 1.)) as usize
    }
}

#[must_use]
pub fn sieve_eratosthenes(bound: usize) -> Vec<usize> {
    let n = bound;
    let mut is_prime = vec![true; n / 2 + 1];
    is_prime[0] = false;
    let mut i = 3;
    while i * i <= n {
        if is_prime[i / 2] {
            let mut j = i * i;
            while j <= n {
                if j & 1 != 0 {
                    is_prime[j / 2] = false;
                }
                j += i;
            }
        }

        i += 2;
    }
    let mut primes: Vec<usize> = is_prime
        .into_iter()
        .enumerate()
        .filter_map(|(num, is_prime)| {
            let num = num * 2 + 1;
            if is_prime && num <= bound {
                Some(num)
            } else {
                None
            }
        })
        .collect();
    if bound >= 2 {
        primes.insert(0, 2);
    }
    primes
}

fn sieve_segment(primes: &[usize], mut lower_bound: usize, upper_bound: usize) -> Vec<usize> {
    if lower_bound & 1 == 0 {
        lower_bound += 1;
    }
    let mut is_prime = vec![true; (upper_bound - lower_bound) / 2 + 1];
    for prime in primes {
        let mut value = (prime * prime).max((lower_bound + prime - 1) / prime * prime);
        while value <= upper_bound {
            if value & 1 != 0 {
                is_prime[(value - lower_bound) / 2] = false;
            }
            value += prime;
        }
    }
    if lower_bound == 1 {
        is_prime[0] = false;
    }
    is_prime
        .into_iter()
        .enumerate()
        .filter_map(|(num, is_prime)| {
            let num = num * 2 + lower_bound;
            if is_prime && num <= upper_bound {
                Some(num)
            } else {
                None
            }
        })
        .collect()
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]
#[must_use]
pub fn block_sieve(bound: usize) -> Vec<usize> {
    static BLOCK_SIZE: usize = 200_000;
    if (bound as f64 * 0.9) as usize <= BLOCK_SIZE {
        // We're going to allow for the value to be
        // off by up to 10% and still use a simple
        // sieve, as at that size it shouldnt matter
        return sieve_eratosthenes(bound);
    }

    let nsqurt = (bound as f64).sqrt() as usize;
    let mut primes = if nsqurt <= BLOCK_SIZE {
        sieve_eratosthenes(nsqurt)
    } else {
        block_sieve(nsqurt)
    };

    let mut completed = nsqurt;
    let mut block_primes = Vec::new();
    while completed < bound {
        let upper_bound = (completed + BLOCK_SIZE).min(bound);
        block_primes.append(&mut sieve_segment(&primes, completed + 1, upper_bound));
        completed += BLOCK_SIZE;
    }
    primes.append(&mut block_primes);

    primes
}

fn count_items_in_range(primes: &[usize], lower_bound: usize, upper_bound: usize) -> usize {
    primes
        .iter()
        .filter(|x| **x >= lower_bound && **x <= upper_bound)
        .count()
}

#[allow(clippy::cast_precision_loss, clippy::missing_panics_doc)]
#[must_use]
pub fn nth_prime(n: usize) -> usize {
    static LOW_PRIMES: [usize; 5] = [2, 3, 5, 7, 11];
    if n < 6 {
        return LOW_PRIMES[n - 1];
    }
    let mut upper_bound = upper_bound_for_nth_prime(n);
    let mut lower_bound = lower_bound_for_nth_prime(n);
    let counter = LegendrePrimeCounter::new(upper_bound);

    // We're going to split the range once before sieving for primes
    let middle = lower_bound + (upper_bound - lower_bound) / 2;
    if counter.count_primes(middle) >= n {
        upper_bound = middle;
    } else {
        lower_bound = middle;
    }

    let primes = sieve_segment(counter.prime_factors(), lower_bound, upper_bound);
    while count_items_in_range(&primes, lower_bound, upper_bound) > 1 {
        let middle = lower_bound + (upper_bound - lower_bound) / 2;
        if counter.count_primes(middle) >= n {
            upper_bound = middle;
        } else {
            lower_bound = middle;
        }
    }

    *primes
        .iter()
        .find(|x| **x >= lower_bound && **x <= upper_bound)
        .expect("should always have exactly one value left at this point")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 2)]
    #[case(10, 29)]
    #[case(100, 541)]
    #[case(1_000, 7_919)]
    #[case(10_000, 104_729)]
    #[case(100_000, 1_299_709)]
    #[case(1_000_000, 15_485_863)]
    #[case(10_000_000, 179_424_673)]
    fn upper_bounds_testing(#[case] n: usize, #[case] prime: usize) {
        assert!(upper_bound_for_nth_prime(n) >= prime);
    }

    #[rstest]
    #[case(1, 2)]
    #[case(10, 29)]
    #[case(100, 541)]
    #[case(1_000, 7_919)]
    #[case(10_000, 104_729)]
    #[case(100_000, 1_299_709)]
    #[case(1_000_000, 15_485_863)]
    #[case(10_000_000, 179_424_673)]
    fn search_testing(#[case] n: usize, #[case] prime: usize) {
        assert_eq!(nth_prime(n), prime);
    }

    #[rstest]
    #[case(1, 2)]
    #[case(10, 29)]
    #[case(100, 541)]
    #[case(1_000, 7_919)]
    #[case(10_000, 104_729)]
    #[case(100_000, 1_299_709)]
    #[case(1_000_000, 15_485_863)]
    #[case(10_000_000, 179_424_673)]
    fn lower_bounds_testing(#[case] n: usize, #[case] prime: usize) {
        assert!(lower_bound_for_nth_prime(n) <= prime);
    }

    #[rstest]
    #[case(1)]
    #[case(10)]
    #[case(100)]
    #[case(1_000)]
    #[case(10_000)]
    #[case(100_000)]
    #[case(1_000_000)]
    #[case(10_000_000)]
    fn method_test(#[case] n: usize) {
        let bound = upper_bound_for_nth_prime(n);
        let block = block_sieve(bound);
        let eratosthenes = sieve_eratosthenes(bound);
        assert_eq!(eratosthenes.len(), block.len());
        assert_eq!(eratosthenes, block);
    }

    #[rstest]
    #[case(100)]
    #[case(1_300_000)]
    fn sanity_check(#[case] n: usize) {
        #[allow(
            clippy::cast_precision_loss,
            clippy::cast_sign_loss,
            clippy::cast_possible_truncation
        )]
        let nsqurt = (n as f64).sqrt() as usize;
        let mut primes = sieve_eratosthenes(nsqurt);
        let mut segment = sieve_segment(&primes, nsqurt + 1, n);
        primes.append(&mut segment);
        let eratosthenes = sieve_eratosthenes(n);
        assert_eq!(eratosthenes, primes);
    }
}
