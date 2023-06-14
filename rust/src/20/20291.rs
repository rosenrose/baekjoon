use std::collections::HashMap;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.lines();
    let mut output = String::new();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut ext_counts = HashMap::with_capacity(n >> 1);

    for filename in input {
        let ext = filename.split('.').next_back().unwrap();
        ext_counts.entry(ext).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut ext_counts = Vec::from_iter(ext_counts);
    ext_counts.sort_unstable();

    for (ext, count) in ext_counts {
        writeln!(output, "{ext} {count}").unwrap();
    }

    print!("{output}");
}
