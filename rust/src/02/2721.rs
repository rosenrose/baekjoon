use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let t = |n: usize| n * (n + 1) / 2;
    let w = (1..301).fold(vec![0], |mut acc, i| {
        acc.push(acc.last().unwrap() + i * t(i + 1));
        acc
    });

    for n in input.skip(1) {
        writeln!(output, "{}", w[n]).unwrap();
    }

    print!("{output}");
}
