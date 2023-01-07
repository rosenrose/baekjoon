use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let n = input();
    let mut current = input();
    let mut max = current;

    for _ in 0..n {
        let (into, out) = (input(), input());
        current = current + into - out;

        if current < 0 {
            println!("0");
            return;
        }

        max = current.max(max);
    }

    println!("{max}");
}
