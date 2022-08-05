use std::collections::HashSet;

fn main() {
    // Since the product will be between 100^2 and 999^2, it has 5-6 digits.
    // In this range there are only 1800 palindromes, which can be pre
    // calculated in order to check fast if a product is a palindrome or not.
    // Not sure if this really brings a lot.
    let mut palindromes = HashSet::new();

    for i in 1..=9 {
        for j in 0..=9 {
            for k in 0..=9 {
                let p1 = i * 100001 + j * 10010 + k * 1100;
                let p2 = i * 10001 + j * 1010 + k * 100;
                palindromes.insert(p1);
                palindromes.insert(p2);
            } 
        }
    }

    let mut max_palindrome = 0;

    for i in (100..=999).rev() {
        for j in (i..=999).rev() {
            let product = i * j;
            if product < max_palindrome {
                break;
            }
            max_palindrome = match palindromes.get(&product) {
                Some(&n) => n,
                None => max_palindrome,
            }
        }
    }

    println!("Problem 4 solution: {}", max_palindrome);
}

