use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let cut_count = (n as f64 * 0.15).round() as usize;

    if n == 0 {
        println!("0");
        return;
    }

    let mut scores: Vec<_> = input.collect();
    scores.sort_unstable();

    let sum: i32 = scores[cut_count..n - cut_count].iter().sum();
    let avg = (sum as f64 / (n - cut_count * 2) as f64).round();

    println!("{avg}");
}
