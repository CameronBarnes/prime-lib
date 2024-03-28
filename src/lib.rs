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

#[must_use]
pub fn sieve_eratosthenes1(bound: usize) -> Vec<usize> {
    let n = bound;
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }

        if i > 2 {
            i += 2;
        } else {
            i += 1;
        }
    }
    is_prime
        .into_iter()
        .enumerate()
        .filter_map(|(num, is_prime)| if is_prime { Some(num) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 2)]
    #[case(10, 31)]
    #[case(100, 613)]
    #[case(1_000, 8_840)]
    #[case(10_000, 104_921)]
    #[case(100_000, 1_301_789)]
    #[case(1_000_000, 15_502_802)]
    #[case(10_000_000, 179_595_382)]
    fn upper_bounds_testing(#[case] n: usize, #[case] expected: usize) {
        assert_eq!(upper_bound_for_nth_prime(n), expected);
    }

    #[rstest]
    #[case(10, 29)]
    #[case(100, 541)]
    #[case(1_000, 7_919)]
    #[case(10_000, 104_729)]
    #[case(100_000, 1_299_709)]
    #[case(1_000_000, 15_485_863)]
    #[case(10_000_000, 179_424_673)]
    fn sieve_testing(#[case] n: usize, #[case] expected: usize) {
        let upper_bound = upper_bound_for_nth_prime(n);
        let primes = sieve_eratosthenes1(upper_bound);
        assert_eq!(primes[n - 1], expected);
    }
}
