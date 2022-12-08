use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    loop {
        let sum = input.next().unwrap() + input.next().unwrap();

        if sum == 0 {
            break;
        }

        println!("{sum}");
    }
}
