use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();
    let mut input = || input.next().unwrap();

    let n: i32 = input().parse().unwrap();
    let (before, after) = (input(), input());

    for (x, y) in before.chars().zip(after.chars()) {
        if matches!((n % 2, x == y), (0, false) | (1, true)) {
            println!("Deletion failed");
            return;
        }
    }

    println!("Deletion succeeded");
}
