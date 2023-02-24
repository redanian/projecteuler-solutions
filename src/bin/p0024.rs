use itertools::Itertools;

fn main() {
    let result: String = (0..=9)
        .permutations(10)
        .nth(999_999)
        .expect("There are not so many permutation for this sequence.")
        .into_iter()
        .map(|n| n.to_string())
        .collect();

    println!("Problem 24 solution: {}", result);
}
