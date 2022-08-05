use primal::Primes;

fn main() {
    let limit = 20;
    let mut product = 1;

    for i in Primes::all().take_while(|i| i <= &limit) {
        // The biggest power of i < limit 
        let mut prime_power = 1;
        loop {
            if prime_power * i > limit {
                break;
            }
            prime_power *= i;
        }
        product *= prime_power;
    }

    println!("Problem 5 solution: {}", product);
}

