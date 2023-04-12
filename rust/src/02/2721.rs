use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.lines().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let t = |n: usize| n * (n + 1) / 2;
    let mut w = [0; 301];

    for i in 1..=300 {
        w[i] = w[i - 1] + (i * t(i + 1));
    }

    for n in input.skip(1) {
        writeln!(output, "{}", w[n]).unwrap();
    }

    print!("{output}");
}
