use primal::Sieve;

fn main() {
    // https://mathworld.wolfram.com/Prime-GeneratingPolynomial.html

    let prime_resolver = Sieve::new(12989);
    let mut n_max = 0;
    let mut a_max = 0;
    let mut b_max = 0;

    // No optimizations necessary as result is calculated fast enough.
    for a in -999..=999 {
        for b in -1000..=1000 {
            let mut generating_primes = true;
            let mut n_max_current = 0;
            for n in 0.. {
                let value = n * (n + a) + b;

                generating_primes &= value > 0 && prime_resolver.is_prime(value as usize);

                if !generating_primes {
                    n_max_current = if n > 0 { n - 1 } else { 0 };
                    break;
                }
            }

            if n_max_current > n_max {
                n_max = n_max_current;
                a_max = a;
                b_max = b;
            }
        }
    }

    println!("Problem 27 solution: {}", a_max * b_max);
}
