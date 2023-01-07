use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    for _ in 0..input.next().unwrap() {
        let n = input.next().unwrap();
        let scores: Vec<_> = (0..n).map(|_| input.next().unwrap()).collect();
        let len = scores.len();

        let sum: i32 = scores.iter().sum();
        let avg = sum as f64 / len as f64;

        let over_avg_count = scores.iter().filter(|&s| *s as f64 > avg).count();
        let over_avg_ratio = over_avg_count as f64 / len as f64;

        println!("{:.3}%", over_avg_ratio * 100.0);
    }
}
