fn main() {
    let mut result = 0;
   
    // a + b + c = 1000, a^2 + b^2 = c^2
    // Find a*b*c through brute force, select a, b, c so that a <= b < c
    // c > a, b     =>  a, b < 500 
    // a <= b < c   =>  a < 334
    
    for a in 1..334 {
        for b in a..500 {
            let c = 1000 - a - b;
            if b < c && a*a + b*b == c*c {
                result = a * b * c;
            }
        }
    }

    println!("Problem 9 solution: {}", result);
}
