use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut input = || input.next();

    while let (Some(a), Some(b), Some(c), Some(d)) = (input(), input(), input(), input()) {
        if (a, b, c, d) == (0, 0, 0, 0) {
            return;
        }

        let ages = [c - a, c - b, d - a, d - b];

        println!(
            "{} {}",
            ages.iter().min().unwrap(),
            ages.iter().max().unwrap()
        );
    }
}
