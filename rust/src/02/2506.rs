use std::io;

const MAX: usize = 100;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut scores = [0; MAX];

    for (i, score) in input.enumerate() {
        scores[i] = score;
    }

    let score: usize = scores[..n]
        .split(|&s| s == 0)
        .flat_map(|chunk| chunk.iter().enumerate().map(|(i, _)| i + 1))
        .sum();

    println!("{score}");
}
