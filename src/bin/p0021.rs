fn main() {
    let result = (2..10000)
        .filter(|a| {
            let b = a.divisors().into_iter().sum::<u32>();
            *a != b && *a == b.divisors().into_iter().sum::<u32>()
        })
        .sum::<u32>();

    println!("Problem 21 solution: {}", result);
}

trait HasDivisors {
    fn divisors(&self) -> Vec<Self> where Self: Sized;
}

impl HasDivisors for u32 {
    fn divisors(&self) -> Vec<u32> {
        let n = *self;
        match n {
            0 | 1 => vec![],
            // Bug: square root of squares is not included.
            _ => (2..((n as f64).sqrt().ceil() as u32))
                .fold(vec![1], |acc, i| if n % i == 0 { [acc, vec![i, n / i]].concat() } else { acc })
        }
    }
}
