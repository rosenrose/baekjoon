use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i64>);

    let mut coords: Vec<_> = input.skip(1).collect();
    coords.sort_unstable();

    let sum: i64 = coords
        .iter()
        .enumerate()
        .map(|(i, num)| 2 * num * (i - (coords.len() - i - 1)) as i64)
        .sum();

    println!("{sum}");
}
