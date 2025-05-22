use std::fmt::Write;
use std::io;

const MAX: usize = 100_000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut friends = [0; MAX];

    for [a, b] in (0..m).map(|_| [(); 2].map(|_| input.next().unwrap() - 1)) {
        friends[a] += 1;
        friends[b] += 1;
    }

    for f in &friends[..n] {
        writeln!(output, "{f}").unwrap();
    }

    print!("{output}");
}
