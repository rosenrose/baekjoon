use std::io;

const MAX: usize = 1000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut scores = [0; MAX];

    for (i, num) in input.enumerate() {
        scores[i] = num;
    }

    let max = *scores[..n].iter().max().unwrap();
    let new_scores = scores[..n].iter().map(|&s| (s as f64 / max as f64) * 100.0);
    let sum: f64 = new_scores.sum();

    println!("{:.10}", sum / n as f64);
}
