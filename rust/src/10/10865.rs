use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let [n, m] = [(); 2].map(|_| input.next().unwrap());
    let mut friends = vec![0; n];

    for [a, b] in (0..m).map(|_| [(); 2].map(|_| input.next().unwrap() - 1)) {
        friends[a] += 1;
        friends[b] += 1;
    }

    for f in friends {
        writeln!(output, "{f}").unwrap();
    }

    print!("{output}");
}
