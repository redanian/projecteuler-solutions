fn main() {
    let n: u64 = 600851475143;
    let result = largest_prime_factor(n).unwrap();
    println!("Problem 3 solution: {}", result);
}

// Get the largest prime factor, if it exists.
fn largest_prime_factor(n: u64) -> Option<u64> {
    match n {
        0 | 1 => None,
        _ => factorize(n).into_iter().max(),
    }
}

// Factorize a number via brute force in O(sqrt(n)) time. Method may factorize
// integers with up to 20 digits (all u64 integers) in realistic time.
// Returns a vector containing prime factors.
fn factorize(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    let mut product = n;

    // A number n has at most 1 prime factor that is greater that sqrt(n). So
    // we start dividing with 2 and work our way up to sqrt(n). If what remains
    // if greater than 1, that is the only prime factor greater then sqrt(n).
    // Since all factors will be prime, we use the primality 6k+-1
    // optimization.

    remove_factor(&mut product, 2, &mut factors);
    remove_factor(&mut product, 3, &mut factors);
    let mut factor: u64 = 5;
    let sqrt = (n as f64).sqrt().ceil() as u64;
    while factor <= sqrt {
        remove_factor(&mut product, factor, &mut factors);
        remove_factor(&mut product, factor + 2, &mut factors);
        factor += 6;
    }
    if product > 1 {
        factors.push(product);
    }
    factors
}

// Remove instances of a factor from a product and push them in an accumulator.
fn remove_factor(product: &mut u64, factor: u64, accumulator: &mut Vec<u64>) {
    while *product % factor == 0 {
        *product /= factor;
        accumulator.push(factor)
    }
}
