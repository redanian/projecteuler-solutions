use prime_factorization::Factorization;

fn main() {
    let solution = combinations(40, 20);
    println!("Problem 15 solution: {}", solution);
}

// Calculate c(n, r) using factorization and replacing division with factor elimination.
fn combinations(n: u128, r: u128) -> u128 {
    // c(n, r) = n! / (n-r)! * r! = ((n-r+1)*(n-r+2)*...*n) / r!
    // Factorize all multipliers into prime factors.
    let top: Vec<u128> = ((n - r + 1)..=n)
        .flat_map(|x| Factorization::<u128>::run(x).factors)
        .collect();
    let bottom: Vec<u128> = (2..=r)
        .flat_map(|x| Factorization::<u128>::run(x).factors)
        .collect();
    // Simulate division using prime factor list subtraction.
    return subtract(top, bottom).iter().product();
}

// List subtraction.
fn subtract(a: Vec<u128>, b: Vec<u128>) -> Vec<u128> {
    let mut result = vec![];
    let mut sorted_a = a.clone(); // Sorted copy of parameter a.
    let mut sorted_b = b.clone(); // Sorted copy of parameter b.
    sorted_a.sort();
    sorted_b.sort();
    let mut index_a: usize = 0;
    let mut index_b: usize = 0;
    // Push (a - b) elements in the result vector.
    // Consecutively insert elements from a to result but skip the ones that also exist in b.
    loop {
        if sorted_a.len() <= index_a {
            // No more elements to loop through.
            break;
        }
        let current_a = sorted_a[index_a];
        if sorted_b.len() <= index_b {
            result.push(current_a);
            index_a += 1;
        } else {
            let current_b = sorted_b[index_b];
            if current_b < current_a {
                index_b += 1;
            } else if current_b > current_a {
                result.push(current_a);
                index_a += 1;
            } else {
                index_a += 1;
                index_b += 1;
            }
        }
    }
    return result;
}
