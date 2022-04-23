fn main() {
    let limit: u64 = 100;

    let left: u64 = (limit * (limit + 1) / 2).pow(2);
    // Alternatively the sum of squares formula could also be used for the right part
    let right: u64 = (1..=limit).map(|i| i.pow(2)).sum();

    let result = left - right;

    println!("Problem 6 solution: {}", result);
}

