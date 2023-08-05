use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);
    let mut output = String::new();

    let [n, _, v] = [(); 3].map(|_| input.next().unwrap() as usize);
    let nodes: Vec<_> = input.by_ref().take(n).collect();
    let offset = v - 1;
    let cycle_len = n - offset;

    for k in input.map(|k| k as usize) {
        if k < offset {
            writeln!(output, "{}", nodes[k])
        } else {
            writeln!(output, "{}", nodes[((k - offset) % cycle_len) + offset])
        }
        .unwrap();
    }

    print!("{output}");
}
