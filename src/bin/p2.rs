fn main() {
    let result: u32 = Fibonacci::new()
        .take_while(|x| *x < 4_000_000)
        .filter(|x| x % 2 == 0)
        .sum();
    println!("Problem 2 solution: {}", result);
}

struct Fibonacci {
    current: u32,
    next: u32,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}
impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.next;
        self.next += self.current;
        self.current = temp;
        Some(self.current)
    }
}
