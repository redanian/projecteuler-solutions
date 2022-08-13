fn main() {
    let collatz = CollatzLength::new();
    let solution = collatz
        .take(1_000_000)
        .reduce(|(x, lx), (y, ly)| if lx > ly { (x, lx) } else { (y, ly) })
        .unwrap()
        .0;
    println!("Problem 14 solution: {}", solution);
}

struct CollatzLength {
    index: usize,
    cache: Vec<usize>,
}

impl CollatzLength {
    fn new() -> CollatzLength {
        CollatzLength {
            index: 1,
            cache: vec![0; 4_000_000],
        }
    }

    fn get_length(&mut self, index: usize) -> usize {
        if index == 1 {
            return 1;
        }
        let use_cache = index < self.cache.len();
        if use_cache {
            let cached = self.cache[index];
            if cached != 0 {
                return cached;
            }
        }
        let value;
        if index % 2 == 0 {
            value = 1 + self.get_length(index / 2);
        } else {
            value = 2 + self.get_length((3 * index + 1) / 2)
        }
        if use_cache {
            self.cache[index] = value;
        }
        return value;
    }
}

impl Iterator for CollatzLength {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        return Some((index, self.get_length(index)));
    }
}
