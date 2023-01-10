use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut baskets: Vec<_> = (1..=n).collect();

    for (mut i, mut j) in (0..m).map(|_| (input() - 1, input() - 1)) {
        while i < j {
            baskets.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    for b in baskets {
        print!("{b} ");
    }
}
