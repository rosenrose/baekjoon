use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let count = buf
        .lines()
        .skip(1)
        .map(|s| s.parse::<i32>().unwrap() - 1)
        .sum::<i32>()
        + 1;

    print!("{count}");
}
