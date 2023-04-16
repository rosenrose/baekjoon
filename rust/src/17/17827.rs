use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();

    let (n, _, v) = (input.next().unwrap(), input.next(), input.next().unwrap());
    let nodes: Vec<_> = input.by_ref().take(n).collect();
    let offset = v - 1;
    let cycle_len = n - offset;

    for k in input {
        if k < offset {
            writeln!(output, "{}", nodes[k])
        } else {
            writeln!(output, "{}", nodes[((k - offset) % cycle_len) + offset])
        }
        .unwrap();
    }

    print!("{output}");
}
