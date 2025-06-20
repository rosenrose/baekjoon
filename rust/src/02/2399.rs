use std::io;

const MAX: usize = 10000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = input.next().unwrap();
    let mut coords = [0; MAX];

    for (i, num) in input.enumerate() {
        coords[i] = num;
    }

    coords[..n].sort_unstable();

    let sum: usize = coords[..n]
        .iter()
        .enumerate()
        .map(|(i, num)| 2 * num * (i - (n - i - 1)))
        .sum();

    println!("{sum}");
}
