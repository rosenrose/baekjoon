use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    for (i, (a, b)) in (1..=input()).map(|i| (i, (input(), input()))) {
        println!("Case #{i}: {}", a + b);
    }
}
