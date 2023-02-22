use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut baskets: Vec<_> = (1..=n).collect();

    for (i, j, k) in (0..m).map(|_| (input(), input(), input())) {
        baskets[i - 1..j].rotate_left(k - i);
    }

    for b in baskets {
        print!("{b} ");
    }
}
