use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let mut balls = vec![0; n];

    for (i, j, k) in (0..m).map(|_| (input() - 1, input() - 1, input())) {
        for idx in i..=j {
            balls[idx] = k;
        }
    }

    for b in balls {
        print!("{b} ");
    }
}
