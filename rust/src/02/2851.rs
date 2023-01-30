use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    let sum_accum: Vec<_> = input
        .scan(0, |acc, score| {
            *acc += score;
            Some(*acc)
        })
        .collect();

    let score = sum_accum
        .iter()
        .rev()
        .min_by_key(|s| s.abs_diff(100))
        .unwrap();

    println!("{score}");
}
