fn main() {
    let result: u32 = (0_u32..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum();
    println!("Problem 1 solution: {}", result);
}
