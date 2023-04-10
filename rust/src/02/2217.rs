use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<i32>);

    let mut weights: Vec<_> = input.skip(1).collect();
    weights.sort_unstable();

    let max_weight = weights
        .iter()
        .enumerate()
        .map(|(i, w)| w * (weights.len() - i) as i32)
        .max()
        .unwrap();

    println!("{max_weight}");
}
