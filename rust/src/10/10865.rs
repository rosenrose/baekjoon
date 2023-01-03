use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut input = || input.next().unwrap();
    let mut output = String::new();

    let (n, m) = (input(), input());
    let friends = (0..m).fold(vec![0; n], |mut acc, _| {
        let (a, b) = (input() - 1, input() - 1);
        acc[a] += 1;
        acc[b] += 1;

        acc
    });

    for f in friends {
        writeln!(output, "{f}").unwrap();
    }

    print!("{output}");
}
