use num_bigint::BigUint;

fn main() {
    let result = (1..100)
        .map(|n: u8| BigUint::from(n))
        .product::<BigUint>()
        .to_string()
        .chars()
        .map(|c: char| c.to_digit(10).unwrap())
        .sum::<u32>();

    println!("Problem 20 solution: {}", result);
}
