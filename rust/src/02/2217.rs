use std::io;

const MAX: usize = 100_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines().flat_map(str::parse::<i32>);

    let n = input.next().unwrap() as usize;
    let mut weights = [0; MAX];

    for (i, num) in input.enumerate() {
        weights[i] = num;
    }

    weights[..n].sort_unstable();

    let max_weight = weights[..n]
        .iter()
        .enumerate()
        .map(|(i, w)| w * (n - i) as i32)
        .max()
        .unwrap();

    println!("{max_weight}");
}
