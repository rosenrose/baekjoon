use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let mut input = || input.next().unwrap();

    let coords: Vec<_> = (0..input()).map(|_| (input(), input())).collect();
    let len: u32 = (0..coords.len())
        .map(|i| {
            let (a, b) = if i == 0 {
                (coords[0], *coords.last().unwrap())
            } else {
                (coords[i - 1], coords[i])
            };

            (a.0.abs_diff(b.0)).max(a.1.abs_diff(b.1))
        })
        .sum();

    println!("{len}");
}
