use std::io::{stdin, Read};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let mut stack = Vec::new();

    for num in input.skip(1) {
        if num == 0 {
            stack.pop();
        } else {
            stack.push(num);
        }
    }

    println!("{}", stack.iter().sum::<i32>());
}
