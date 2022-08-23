use num_bigint::BigUint;

fn main() {
    let two = BigUint::from(2u32);
    let power = two.pow(1000);
    let solution: u32 = power
        .to_string()
        .chars()
        .map(|c: char| c.to_digit(10).unwrap())
        .sum();
    println!("Problem 16 solution: {}", solution);
}
