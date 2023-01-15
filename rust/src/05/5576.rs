use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    let mut w_scores: Vec<_> = (0..10).map(|_| input.next().unwrap()).collect();
    w_scores.sort();
    let w_sum: i32 = w_scores[7..].iter().sum();

    let mut k_scores: Vec<_> = input.collect();
    k_scores.sort();
    let k_sum: i32 = k_scores[7..].iter().sum();

    println!("{w_sum} {k_sum}");
}
