use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    let mut sum_accum = [0; 11];

    for (i, score) in input.enumerate() {
        sum_accum[i + 1] = sum_accum[i] + score;
    }

    let score = sum_accum
        .iter()
        .rev()
        .min_by_key(|s| s.abs_diff(100))
        .unwrap();

    println!("{score}");
}
