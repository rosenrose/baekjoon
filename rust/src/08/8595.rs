use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let hidden_nums = buf
        .lines()
        .next_back()
        .unwrap()
        .split(char::is_alphabetic)
        .filter_map(|s| s.parse::<i64>().ok());

    println!("{}", hidden_nums.sum::<i64>());
}
