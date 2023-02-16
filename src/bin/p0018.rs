use std::cmp::max;
use std::fs;

fn main() {
    let triangle_file = "resources/p0018/triangle.txt";
    let file_content = fs::read_to_string(triangle_file).expect(&format!("Could not read file {}", triangle_file));
    let result = NumbersTriangle::from(file_content).max_total();
    println!("Problem 18 solution: {}", result);
}

struct NumbersTriangle {
    numbers: Vec<u32>,
}

impl NumbersTriangle {
    fn from(string: String) -> Self {
        NumbersTriangle {
            numbers: string
                .split_ascii_whitespace()
                .rev()
                .map(|s| s.parse::<u32>().expect(&format!("Could not parse {s} as u32.")))
                .collect::<Vec<u32>>(),
        }
    }

    fn max_total(&self) -> u32 {
        if self.numbers.is_empty() {
            panic!("Triangle is empty")
        }
        let height = self.triangle_height();
        if (height * (height + 1) / 2) != self.numbers.len() {
            panic!("Input not a valid triangle")
        }
        /*
        Example triangle:

        9
        8 7
        6 5 4
        3 2 1 0

        size = 10, height = 4
        Algorithm to propagate max to the top:

        4 += max (0 1)
        5 += max (1 2)
        6 += max (2 3)

        7 += max (4 5)
        8 += max (5 6)

        9 += max (7 8)
         */
        let mut numbers = self.numbers.clone();
        let mut counter = 0;
        let mut current_step = 0;
        let mut max_steps = height - 1;
        for i in height..self.numbers.len() {
            numbers[i] += max(numbers[counter], numbers[counter + 1]);
            counter += 1;
            current_step += 1;
            if current_step == max_steps {
                current_step = 0;
                max_steps -= 1;
                counter += 1;
            }
        }
        *numbers.last().unwrap()
    }

    fn triangle_height(&self) -> usize {
        let mut size = self.numbers.len() as i32;
        let mut height = 0;
        for i in 0.. {
            size -= i;
            if !size.is_positive() {
                height = i as usize;
                break;
            }
        };
        height
    }
}