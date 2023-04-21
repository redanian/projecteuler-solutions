use std::mem::replace;

use num_bigint::BigUint;

fn main() {
    let result: u32 = BigFibonacci::new()
        .zip(1..)
        .find(|(fib, _)| fib.to_string().len() >= 1000)
        .expect("No number satisfies the required predicate.")
        .1;
    println!("Problem 25 solution: {}", result);
}

struct BigFibonacci {
    prev: BigUint,
    current: BigUint,
}

impl BigFibonacci {
    fn new() -> Self {
        BigFibonacci {
            prev: BigUint::from(0u8),
            current: BigUint::from(1u8),
        }
    }
}

impl Iterator for BigFibonacci {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.current.clone();
        let next = &self.prev + &self.current;
        self.prev = replace(&mut self.current, next);
        return Some(ret);
    }
}
