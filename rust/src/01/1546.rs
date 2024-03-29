use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let scores: Vec<_> = input.skip(1).collect();
    let max = *scores.iter().max().unwrap();

    let new_scores = scores.iter().map(|&s| (s as f64 / max as f64) * 100.0);
    let sum: f64 = new_scores.sum();

    println!("{:.10}", sum / scores.len() as f64);
}
