use primal::Primes;

fn main() {
    let limit: usize = 2_000_000;
    let sum: usize = Primes::all().take_while(|n| n <= &limit).sum();
    println!("Problem 10 solution: {}", sum);
}

