fn main() {
    // Helpful: https://mathworld.wolfram.com/FullReptendPrime.html
    let result = (1..1000)
        .max_by_key(|&n| get_unit_fraction_cycle_length(n))
        .unwrap();

    println!("Problem 26 solution: {}", result);
}

/// Get the cycle length of a unit fraction by brute forcing through the remainders. A cycle will begin only if a
/// remainder is repeated.
fn get_unit_fraction_cycle_length(n: usize) -> usize {
    let mut remainder = 1;
    let mut known_remainders = vec![];

    while remainder != 0 && !known_remainders.contains(&remainder) {
        known_remainders.push(remainder);
        remainder = (remainder * 10) % n;
    }

    match remainder {
        0 => 0,
        _ => {
            let index = known_remainders.iter().position(|&x| x == remainder).unwrap();
            known_remainders.len() - index
        }
    }
}

/// Produces the sequence of digits after the decimal point for decimal unit fractions.
///
/// A decimal unit fraction is of the form `1/n` where `n` is a positive integer. This struct simulates the process of
/// long division to extract the sequence of digits after the decimal point.
///
/// # Example
///
/// For `n = 6`, the decimal representation is `0.1(6)`, therefore, the producer will generate the sequence `166666...`.
struct DecimalUnitFractionProducer {
    n: usize,
    remainder: usize,
}

impl DecimalUnitFractionProducer {
    fn new(n: usize) -> DecimalUnitFractionProducer {
        DecimalUnitFractionProducer {
            n,
            remainder: 1,
        }
    }
}

impl Iterator for DecimalUnitFractionProducer {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.remainder {
            0 => None,
            _ => {
                self.remainder *= 10;
                let quotient = self.remainder / self.n;
                self.remainder = self.remainder % self.n;

                Some(quotient)
            }
        }
    }
}
