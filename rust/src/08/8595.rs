use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let hidden_nums = buf
        .lines()
        .next_back()
        .unwrap()
        .split(char::is_alphabetic)
        .filter_map(|s| s.parse::<i64>().ok());

    println!("{}", hidden_nums.sum::<i64>());
}
