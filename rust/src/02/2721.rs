use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let t = |n: usize| n * (n + 1) / 2;
    let w: Vec<_> = (0..=300)
        .scan(0, |acc, i| {
            *acc += i * t(i + 1);
            Some(*acc)
        })
        .collect();

    for n in input.skip(1) {
        writeln!(output, "{}", w[n]).unwrap();
    }

    print!("{output}");
}
