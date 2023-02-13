fn main() {
    let result: usize = (1..=1000)
        .map(|i: u32| i.in_words())
        .map(|s: String| s.chars().filter(|c: &char| *c != ' ' && *c != '-').collect())
        .map(|s: String| s.len())
        .sum();
    println!("Problem 17 solution: {}", result);
}

trait InWords {
    fn in_words(&self) -> String;
}

impl InWords for u32 {
    fn in_words(&self) -> String {
        match &self {
            0 => String::from("zero"),
            1 => String::from("one"),
            2 => String::from("two"),
            3 => String::from("three"),
            4 => String::from("four"),
            5 => String::from("five"),
            6 => String::from("six"),
            7 => String::from("seven"),
            8 => String::from("eight"),
            9 => String::from("nine"),
            10 => String::from("ten"),
            11 => String::from("eleven"),
            12 => String::from("twelve"),
            13 => String::from("thirteen"),
            14 => String::from("fourteen"),
            15 => String::from("fifteen"),
            16 => String::from("sixteen"),
            17 => String::from("seventeen"),
            18 => String::from("eighteen"),
            19 => String::from("nineteen"),
            20 => String::from("twenty"),
            30 => String::from("thirty"),
            40 => String::from("forty"),
            50 => String::from("fifty"),
            60 => String::from("sixty"),
            70 => String::from("seventy"),
            80 => String::from("eighty"),
            90 => String::from("ninety"),
            21..=99 => format!("{}-{}", (*self / 10 * 10).in_words(), (*self % 10).in_words()),
            100 | 200 | 300 | 400 | 500 | 600 | 700 | 800 | 900 => format!("{} hundred", (*self / 100).in_words()),
            101..=999 => format!("{} hundred and {}", (*self / 100).in_words(), (*self % 100).in_words()),
            1000 => String::from("one thousand"),
            _ => panic!("Cannot describe {} in words :(.", &self)
        }
    }
}
