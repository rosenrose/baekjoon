use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let score: usize = input
        .skip(1)
        .collect::<Vec<_>>()
        .split(|&c| c == 0)
        .flat_map(|s| s.iter().enumerate().map(|(i, _)| i + 1))
        .sum();

    println!("{score}");
}
