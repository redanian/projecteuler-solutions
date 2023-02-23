use std::collections::HashSet;

fn main() {
    let limit: u32 = 28123;

    let abundants_vec: Vec<u32> = (1..=limit).filter(u32::is_abundant).collect();
    let abundants_set: HashSet<u32> = HashSet::from_iter(abundants_vec.clone());

    let result: u32 = (1..=limit)
        .filter(|n| {
            let mut pass = true;
            for a in abundants_vec.iter() {
                if n <= a {
                    break;
                }
                if abundants_set.contains(&(n - a)) {
                    pass = false;
                    break;
                }
            }
            pass
        })
        .sum();

    println!("Problem 23 solution: {}", result);
}

trait IsAbundant {
    fn is_abundant(&self) -> bool;
    fn divisors(&self) -> Vec<Self> where Self: Sized;
}

impl IsAbundant for u32 {
    fn is_abundant(&self) -> bool {
        return &self.divisors().into_iter().sum::<u32>() > self;
    }

    fn divisors(&self) -> Vec<u32> {
        let number = *self;
        match number {
            0 | 1 => vec![],
            _ => (2..=((number as f64).sqrt().floor() as u32))
                .fold(vec![1], |accumulator, i|
                    if i * i == number {
                        [accumulator, vec![i]].concat()
                    } else if number % i == 0 {
                        [accumulator, vec![i, number / i]].concat()
                    } else {
                        accumulator
                    },
                )
        }
    }
}
