use primal::Primes;

fn main() {
    let result = Primes::all().nth(10000).unwrap();

    println!("Problem 7 solution: {}", result);
}
