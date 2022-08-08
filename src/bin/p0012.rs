fn main() {
    let result: u32 = Triangular::new()
        .find(|x| nr_of_divisors(*x) > 500)
        .unwrap();
    println!("Problem 12 solution: {}", result);
}
// Brute force the number of divisors.
fn nr_of_divisors(a: u32) -> u32 {
    let sqrt = (a as f64).sqrt().ceil() as u32;
    return (2 * ((1..sqrt).filter(|i| a % i == 0).count() as u32))
        + if sqrt ^ 2 == a { 1 } else { 0 };
}

struct Triangular {
    n: u32,
    sum: u32,
}

impl Triangular {
    fn new() -> Self {
        Triangular { n: 1, sum: 0 }
    }
}
impl Iterator for Triangular {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.sum += self.n;
        self.n += 1;
        Some(self.sum)
    }
}
