use std::fs;

fn main() {
    let names_file = "resources/p0022/names.txt";
    let file_content = fs::read_to_string(names_file).expect(&format!("Could not read file {}", names_file));

    let mut names = file_content
        .split(",")
        .map(|s| s.trim_matches('"'))
        .collect::<Vec<&str>>();

    names.sort();

    let result = names
        .into_iter()
        .map(|name| name
            .chars()
            .map(|c| c.to_ascii_lowercase())
            .map(<u64 as From<char>>::from)
            .map(|c| c - 96)
            .sum::<u64>()
        )
        .zip(1..)
        .map(|(v, i)| v * i)
        .sum::<u64>();

    println!("Problem 22 solution: {}", result);
}