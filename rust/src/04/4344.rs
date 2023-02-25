use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<f64>);
    let mut input = || input.next().unwrap();

    for _ in 0..input() as i32 {
        let n = input();
        let scores: Vec<_> = (0..n as i32).map(|_| input()).collect();
        let avg = scores.iter().sum::<f64>() / n;

        let over_avg_count = scores.iter().filter(|&&s| s > avg).count();
        let over_avg_ratio = over_avg_count as f64 / n;

        println!("{:.3}%", over_avg_ratio * 100.0);
    }
}
