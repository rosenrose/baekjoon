use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut input = || input.next().unwrap();

    let (n, m) = (input(), input());
    let balls = (0..m).fold(vec![0; n], |mut acc, _| {
        let (i, j, k) = (input() - 1, input() - 1, input());

        for idx in i..=j {
            acc[idx] = k;
        }

        acc
    });

    for b in balls {
        print!("{b} ");
    }
}
