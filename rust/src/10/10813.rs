use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut balls: Vec<_> = (1..=n).collect();

    for (i, j) in (0..m).map(|_| (input() - 1, input() - 1)) {
        balls.swap(i, j);
    }

    for ball in balls {
        print!("{ball} ");
    }
}
